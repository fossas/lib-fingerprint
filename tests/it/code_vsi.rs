//! Tests for plain code files using legacy VSI fingerprints.

use std::collections::HashSet;

use pretty_assertions::assert_eq;

use fingerprint::*;
use strum::IntoEnumIterator;

/// Assert fingerprint for the named kind is the same as the fingerprint for the provided content.
/// Comparison content (the "expected" fingerprint) is generated with [`hash`].
///
/// ```ignore
/// let content = b"hello world";
/// let combined = Combined::from_buffer(&content).expect("fingerprint");
/// assert_fingerprint_eq!(Kind::RawSha256, content, combined);
/// assert_fingerprint_eq!(Kind::CommentStrippedSha256, content, combined);
/// ```
///
/// You can also assert a [`None`] value:
/// ```ignore
/// let content = vec![1, 2, 3, 0, 1, 2, 3];
/// let combined = Combined::from_stream(&mut Cursor::new(content.clone())).expect("fingerprint");
/// assert_fingerprint_eq!(Kind::RawSha256, &content, combined);
/// assert_fingerprint_eq!(Kind::CommentStrippedSha256, None, combined);
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
    let fp = Fingerprint::new(Kind::RawSha256, content.clone());
    assert_eq!(&content, fp.content())
}

#[test]
fn combined_getters() {
    let raw = make_fingerprint(Kind::RawSha256, b"hello world raw");
    let cs = make_fingerprint(Kind::CommentStrippedSha256, b"hello world comment stripped");

    let combined = Combined::from([&raw, &cs]);
    assert_eq!(Some(raw.content()), combined.get(Kind::RawSha256));
    assert_eq!(
        Some(cs.content()),
        combined.get(Kind::CommentStrippedSha256)
    );

    let combined = Combined::single(&raw);
    assert_eq!(Some(raw.content()), combined.get(Kind::RawSha256));
    assert_eq!(None, combined.get(Kind::CommentStrippedSha256));
}

#[test]
fn fingerprints_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = Combined::from_buffer(&content).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RawSha256, &content, combined);
    assert_fingerprint_eq!(Kind::CommentStrippedSha256, None, combined);
}

#[test]
fn fingerprints_text_file() {
    let content = b"hello world";
    let combined = Combined::from_buffer(content).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RawSha256, content, combined);
    assert_fingerprint_eq!(Kind::CommentStrippedSha256, content, combined);
}

#[test]
fn fingerprints_text_file_stripping_cr() {
    let content = b"hello world\r\nanother line\r\na final line\n";
    let content_cs = b"hello world\nanother line\na final line";
    let without_cr = b"hello world\nanother line\na final line\n";

    let combined = Combined::from_buffer(content).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RawSha256, without_cr, combined);
    assert_fingerprint_eq!(Kind::CommentStrippedSha256, content_cs, combined);
}

#[test]
fn fingerprints_binary_file_appearing_as_text() {
    // Sourced from `git@github.com:chromium/chromium.git` at `tools/origin_trials/eftest.key` on commit 49249345609d505c8bb8b0b5a42ff4b68b9e6d41.
    let content = include_bytes!("../../testdata/eftest.key");
    let combined = Combined::from_buffer(content).expect("fingerprint");
    assert_fingerprint_eq!(Kind::RawSha256, content, combined);
    assert_fingerprint_eq!(Kind::CommentStrippedSha256, None, combined);
}

#[test]
fn comment_stripped_does_not_fingerprint_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = Combined::from_buffer(content).expect("fingerprint");
    assert_fingerprint_eq!(Kind::CommentStrippedSha256, None, combined);
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

    let combined = Combined::from_buffer(content).expect("fingerprint");
    let expected = Content::new(
        hex::decode("44fc8f68ab633c7ca0240a66e4ff038c0f2412fe69d14b6f052556edaa1b9160")
            .expect("decode hex literal"),
    );
    assert_eq!(Some(&expected), combined.get(Kind::CommentStrippedSha256));
}

#[test]
fn strum_serde_fingerprint_kind_serialization_matches() {
    for kind in Kind::iter() {
        let json_value = serde_json::to_value(kind).expect("serde serialization");
        let json = json_value.as_str().expect("get string");
        let strum = kind.to_string();

        assert_eq!(json, strum);
    }
}

#[test]
fn serde_serialization_matches_external_contract() {
    let mut serialized_strings = vec![
        "v1.class.jar",
        "v1.mavencentral.jar",
        "v1.raw.jar",
        "comment_stripped:sha_256",
        "sha_256",
    ]
    .into_iter()
    .collect::<HashSet<&str>>();

    for kind in Kind::iter() {
        let json_value = serde_json::to_value(kind).expect("serde serialization");
        let json = json_value.as_str().expect("get string");

        serialized_strings.remove(json);
    }

    assert_eq!(serialized_strings, HashSet::new());
}
