//! Tests for the external API.

use std::io::Cursor;

use pretty_assertions::assert_eq;

use fingerprint::*;

/// Assert fingerprint for the named kind is the same as the fingerprint for the provided content.
/// Comparison content (the "expected" fingerprint) is generated with [`hash`].
///
/// ```ignore
/// let content = b"hello world";
/// let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
/// assert_fingerprint_eq!(Kind::RAW_SHA256, content, combined);
/// assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, content, combined);
/// ```
///
/// You can also assert a [`None`] value:
/// ```ignore
/// let content = vec![1, 2, 3, 0, 1, 2, 3];
/// let combined = fingerprint_stream(&mut Cursor::new(content.clone())).expect("fingerprint");
/// assert_fingerprint_eq!(Kind::RAW_SHA256, &content, combined);
/// assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, None, combined);
/// ```
macro_rules! assert_fingerprint_eq {
    ($kind:expr, None, $combined:ident) => {{
        pretty_assertions::assert_eq!(None, $combined.get($kind));
    }};
    ($kind:expr, $content:expr, $combined:ident) => {{
        let expected = make_fingerprint($kind, $content);
        pretty_assertions::assert_eq!(Some(expected.content()), $combined.get($kind));
    }};
}

/// Create a fingerprint, asserted to the provided kind.
///
/// This is for testing; the hash used may not actually match the hash
/// for the same content with the named kind.
fn make_fingerprint(kind: Kind, content: &[u8]) -> Fingerprint {
    let content = Content::hash_sha256(content);
    Fingerprint::new(kind, content)
}

#[test]
fn fp_getters() {
    let content = Content::hash_sha256(b"hello world");
    let fp = Fingerprint::new(Kind::RAW_SHA256, content.clone());
    assert_eq!(&content, fp.content())
}

#[test]
fn combined_getters() {
    let raw = make_fingerprint(Kind::RAW_SHA256, b"hello world raw");
    let cs = make_fingerprint(
        Kind::COMMENT_STRIPPED_SHA256,
        b"hello world comment stripped",
    );

    let combined = Combined::new([&raw, &cs]);
    assert_eq!(Some(raw.content()), combined.get(Kind::RAW_SHA256));
    assert_eq!(
        Some(cs.content()),
        combined.get(Kind::COMMENT_STRIPPED_SHA256)
    );

    let combined = Combined::single(&raw);
    assert_eq!(Some(raw.content()), combined.get(Kind::RAW_SHA256));
    assert_eq!(None, combined.get(Kind::COMMENT_STRIPPED_SHA256));
}

#[test]
fn fingerprints_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = fingerprint_stream(&mut Cursor::new(content.clone())).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RAW_SHA256, &content, combined);
    assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, None, combined);
}

#[test]
fn fingerprints_text_file() {
    let content = b"hello world";
    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RAW_SHA256, content, combined);
    assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, content, combined);
}

#[test]
fn fingerprints_text_file_stripping_cr() {
    let content = b"hello world\r\nanother line\r\na final line\n";
    let content_cs = b"hello world\nanother line\na final line";
    let without_cr = b"hello world\nanother line\na final line\n";

    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RAW_SHA256, without_cr, combined);
    assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, content_cs, combined);
}

#[test]
fn fingerprints_binary_file_appearing_as_text() {
    // Sourced from `git@github.com:chromium/chromium.git` at `tools/origin_trials/eftest.key` on commit 49249345609d505c8bb8b0b5a42ff4b68b9e6d41.
    let content = include_bytes!("../../testdata/eftest.key");
    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RAW_SHA256, content, combined);
    assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, None, combined);
}

#[test]
fn comment_stripped_does_not_fingerprint_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
    assert_fingerprint_eq!(Kind::COMMENT_STRIPPED_SHA256, None, combined);
}

#[test]
fn comment_stripped_fingerprint_text_file() {
    let content = br#"/*
 * This is a placeholder file used to test comment stripping code.
*/

int main() {
  int code = 0;
  // code = 1;

  return code; // perfect
}
"#;

    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("fingerprint");
    let expected = Content::new(
        hex::decode("44fc8f68ab633c7ca0240a66e4ff038c0f2412fe69d14b6f052556edaa1b9160")
            .expect("decode hex literal"),
    );
    assert_eq!(Some(&expected), combined.get(Kind::COMMENT_STRIPPED_SHA256));
}
