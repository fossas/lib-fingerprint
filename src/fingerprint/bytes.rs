use std::io::{BufRead, Cursor, Read as _, Write};

use sha2::{Digest, Sha256};

use crate::{Error, Fingerprint, Kind};

use super::{binary, text};

/// Fingerprint the file as a raw chunk of bytes.
#[tracing::instrument(level = tracing::Level::DEBUG, skip(stream), ret)]
pub fn raw<R: BufRead>(mut stream: R) -> Result<Fingerprint, Error> {
    // Read the start of the stream, and decide whether to treat the rest of the stream as binary based on that.
    let binary::Check { is_binary, read } = binary::Check::content(&mut stream)?;

    // Chain the part of the stream already read to evaluate binary along with the rest of the stream.
    let mut stream = Cursor::new(read).chain(stream);
    let mut hasher = Sha256::new();
    if is_binary {
        content(&mut stream, &mut hasher)?;
    } else {
        text::content(&mut stream, &mut hasher)?;
    }

    Ok(Fingerprint::from_digest(Kind::RawSha256, hasher))
}

/// Reads the exact contents of a binary file without modification.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn content(stream: &mut impl BufRead, w: &mut impl Write) -> Result<(), Error> {
    std::io::copy(stream, w)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::Content;

    use super::*;

    #[test]
    fn raw_content() {
        let input = b"some content";
        let mut read = Vec::new();
        content(&mut Cursor::new(input), &mut read).expect("read");
        assert_eq!(
            String::from_utf8_lossy(input),
            String::from_utf8_lossy(&read)
        );
    }

    #[test]
    fn text_content() {
        let content = b"hello world\r\nanother line\r\na final line\n";
        let without_cr = b"hello world\nanother line\na final line\n";

        let fp = raw(&mut Cursor::new(content)).expect("fingerprint");
        assert_eq!(&Content::hash_sha256(without_cr), fp.content());
    }

    #[test]
    fn binary_content() {
        let content = vec![1, 2, 3, 0, 1, 2, 3];
        let fp = raw(&mut Cursor::new(&content)).expect("fingerprint");
        assert_eq!(&Content::hash_sha256(content), fp.content());
    }

    #[test]
    fn binary_content_appearing_as_text() {
        let content = include_bytes!("../../testdata/eftest.key");
        let fp = raw(&mut Cursor::new(content)).expect("fingerprint");
        assert_eq!(&Content::hash_sha256(content), fp.content());
    }
}
