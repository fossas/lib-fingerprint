use std::io::{BufRead, BufReader, Cursor, Read as _, Write};

use iter_read::IterRead;
use sha2::{Digest, Sha256};
use tap::Pipe as _;

use crate::{stream::ConvertCRLFToLF as _, Error, Fingerprint, Kind};

use super::binary;

/// Fingerprint the file with basic C-style comment stripping.
///
/// Specifically:
/// - All text encodings are treated as utf8.
/// - `git` implementations on Windows typically check out files with `\r\n` line endings,
///   while *nix checks them out with `\n`.
///   To be platform independent, any `\r\n` byte sequences found are converted to a single `\n`.
/// - C-style comments are removed:
///   - `//` is considered the start of a single line comment; these bytes and any other bytes until right before a `\n` are removed.
///   - `/*` is considered the start of a multi line comment; these bytes and any other bytes until after a `*/` is read are removed.
///   - This function does not check for escaped comments.
/// - Any sequence of multiple contiguous `\n` bytes are collapsed to a single `\n` byte.
/// - The final `\n` byte is removed from the end of the stream if present.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn comment_stripped(mut stream: impl BufRead) -> Result<Option<Fingerprint>, Error> {
    // Read the start of the stream, and decide whether to treat the rest of the stream as binary based on that.
    let binary::Check { is_binary, read } = binary::Check::content(&mut stream)?;
    if is_binary {
        return Ok(None);
    }

    // Chain the part of the stream already read to evaluate binary along with the rest of the stream.
    let mut stream = Cursor::new(read).chain(stream);
    let mut hasher = Sha256::new();
    match content_stripped(&mut stream, &mut hasher) {
        Ok(_) => Ok(Some(Fingerprint::from_digest(
            Kind::CommentStrippedSha256,
            hasher,
        ))),
        Err(err) => {
            // The `io::Error` type is opaque.
            // Handle the case of attempting to comment strip a binary file.
            if err.to_string().to_lowercase().contains("utf-8") {
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

/// Reads text files in a platform independent manner.
///
/// Specifically:
/// - All text encodings are ignored; this function operates on raw bytes.
/// - `git` implementations on Windows typically check out files with `\r\n` line endings,
///   while *nix checks them out with `\n`.
///   To be platform independent, any `\r\n` byte sequences found are converted to a single `\n`.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn content(stream: impl BufRead, mut w: impl Write) -> Result<(), Error> {
    let mut stream = BufReader::new(stream)
        .bytes()
        .crlf_to_lf()
        .fuse()
        .pipe(IterRead::new);
    std::io::copy(&mut stream, &mut w)?;
    Ok(())
}

/// Hashes code files while removing C-style comments and blank lines in a platform independent manner.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
fn content_stripped(stream: &mut impl BufRead, w: &mut impl Write) -> Result<(), Error> {
    let mut buffered_output_line = String::new();
    let mut is_multiline_active = false;

    for line in stream.lines() {
        let mut line = line?;

        // At this point we know we have a new line coming. If a previous line is buffered and ready to write, do so now.
        // Write it with a trailing newline because we know we'll be writing a following line.
        if !buffered_output_line.is_empty() {
            writeln!(w, "{buffered_output_line}")?;
        }

        (line, is_multiline_active) = clean_line(line, is_multiline_active);
        line.trim().clone_into(&mut buffered_output_line);
    }

    // Now that we're done reading the input stream, if there's a buffered output line write it *without a trailing newline*.
    write!(w, "{buffered_output_line}")?;
    Ok(())
}

/// Part comment stripping, part state machine. Cleans lines of comments based on whether a previous invocation
/// detected the start of a multi line comment.
///
/// This is very much not an ideal function: it scans the line multiple times instead of being forward-looking-only,
/// and the dual responsibility makes it complicated. We should fix this, but moving forward for now.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
fn clean_line(line: String, is_multiline_active: bool) -> (String, bool) {
    if is_multiline_active {
        if let Some(end) = line.find("*/") {
            return clean_line(line[end + 2..].to_string(), false);
        }

        (String::new(), true)
    } else if let Some(start) = line.find("/*") {
        let before_multi = line[..start].to_string();
        let (after_multi, is_multi) = clean_line(line[start + 2..].to_string(), true);
        (before_multi + &after_multi, is_multi)
    } else if let Some(start) = line.find("//") {
        (line[..start].to_string(), false)
    } else {
        (line, false)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Inspired by the Haskell implementation: https://github.com/fossas/fossa-cli/blob/8de74b71b80d77321d64f94d7573773e49306772/test/App/Fossa/VSI/testdata/multi_line_comment.c#L1-L10
    #[test]
    fn comment_strip_mixed() {
        let content = r#"/*
 * This is a placeholder file used to test comment stripping code.
*/

int main() {
  int code = 0;
  // code = 1;




  return code; // perfect
}
"#;
        let expected = r#"int main() {
int code = 0;
return code;
}"#;

        let mut buf = Vec::new();
        content_stripped(&mut Cursor::new(content), &mut buf).expect("must fingerprint");
        assert_eq!(expected, String::from_utf8_lossy(&buf));
    }

    /// Copied from the Go implementation: https://github.com/fossas/basis/blob/6b0a1ce7ca5d88d033732f6dcfebd90b8f143038/sherlock/pkg/lib/indexer/cleaned/strip_comments_internal_test.go#L71-L79
    #[test]
    fn comment_strip_single_line_comments() {
        let content = " content1 \n content2 //comment \n content3 ";
        let expected = "content1\ncontent2\ncontent3";

        let mut buf = Vec::new();
        content_stripped(&mut Cursor::new(content), &mut buf).expect("must fingerprint");
        assert_eq!(expected, String::from_utf8_lossy(&buf));
    }

    /// Copied from the Go implementation: https://github.com/fossas/basis/blob/6b0a1ce7ca5d88d033732f6dcfebd90b8f143038/sherlock/pkg/lib/indexer/cleaned/strip_comments_internal_test.go#L89-L97
    #[test]
    fn comment_strip_multi_line_comments() {
        let content =
            " content1 \n  content2 /* begin comment \n end comment */ content3 \n content4 ";
        let expected = "content1\ncontent2\ncontent3\ncontent4";

        let mut buf = Vec::new();
        content_stripped(&mut Cursor::new(content), &mut buf).expect("must fingerprint");
        assert_eq!(expected, String::from_utf8_lossy(&buf));
    }

    #[test]
    fn comment_strip_cr() {
        let content = "hello world\r\nanother line\r\na final line\n";
        let expected = "hello world\nanother line\na final line";

        let mut buf = Vec::new();
        content_stripped(&mut Cursor::new(content), &mut buf).expect("must fingerprint");
        assert_eq!(expected, String::from_utf8_lossy(&buf));
    }

    #[test]
    fn comment_strip_real_source() {
        let content = include_bytes!("../../testdata/facebook-folly-Version.cpp");
        let expected = include_str!("../../testdata/facebook-folly-Version.cpp.stripped");

        let mut buf = Vec::new();
        content_stripped(&mut Cursor::new(content), &mut buf).expect("must process");

        assert_eq!(normalize_lf(expected), String::from_utf8_lossy(&buf));
    }

    /// Windows CI checks out CRLF. Normalize it to be LF only.
    /// This function should only be applied to testing values, not responses from the functions being tested.
    fn normalize_lf(input: impl Into<String>) -> String {
        input.into().replace("\r\n", "\n")
    }
}
