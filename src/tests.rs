//! Tests for the external API.

use std::io::Cursor;

use sha2::{Digest, Sha256};

use crate::serialize::kind::{kinds_evaluated, would_evaluate_new_kinds, SerializedKind};

use super::*;

fn hash(content: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize().as_slice().to_vec()
}

fn make_fingerprint<K: Kind>(content: &[u8]) -> Fingerprint<K> {
    Fingerprint::builder()
        .content(Blob(hash(content)))
        .kind(PhantomData {})
        .build()
}

#[test]
fn fp_getters() {
    let content = Blob(hash(b"hello world"));
    let fp = Fingerprint::builder()
        .content(content.clone())
        .kind(PhantomData::<RawSHA256> {})
        .build();

    assert_eq!(&content, fp.content())
}

#[test]
fn combined_getters() {
    let raw = make_fingerprint::<RawSHA256>(b"hello world raw");
    let comment_stripped =
        make_fingerprint::<CommentStrippedSHA256>(b"hello world comment stripped");
    let combined = Combined::builder()
        .raw(raw.clone())
        .comment_stripped(Some(comment_stripped.clone()))
        .build();

    assert_eq!(&raw, combined.raw());
    assert_eq!(&Some(comment_stripped), combined.comment_stripped());

    let combined = Combined::builder()
        .raw(raw.clone())
        .comment_stripped(None)
        .build();
    assert_eq!(&raw, combined.raw());
    assert_eq!(&None, combined.comment_stripped());
}

#[test]
fn fingerprints_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = fingerprint_stream(&mut Cursor::new(content.clone())).expect("should not error");
    let expected_fingerprint = make_fingerprint::<RawSHA256>(&content);

    assert_eq!(combined.raw, expected_fingerprint);
    assert_eq!(combined.comment_stripped, None);
}

#[test]
fn fingerprints_text_file() {
    let content = b"hello world";

    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("should not error");
    let expected_fingerprint = make_fingerprint::<RawSHA256>(content);
    assert_eq!(combined.raw, expected_fingerprint);

    let expected_fingerprint = make_fingerprint::<CommentStrippedSHA256>(content);
    assert_eq!(combined.comment_stripped, Some(expected_fingerprint));
}

#[test]
fn fingerprints_text_file_stripping_cr() {
    let content = b"hello world\r\nanother line\r\na final line\n";
    let cr_stripped = b"hello world\nanother line\na final line\n";

    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("should not error");
    let expected_fingerprint = make_fingerprint::<RawSHA256>(cr_stripped);
    assert_eq!(combined.raw, expected_fingerprint, "raw");

    let comment_stripped = b"hello world\nanother line\na final line";
    let expected_fingerprint = make_fingerprint::<CommentStrippedSHA256>(comment_stripped);
    assert_eq!(
        combined.comment_stripped,
        Some(expected_fingerprint),
        "comment stripped"
    );
}

#[test]
fn fingerprints_binary_file_appearing_as_text() {
    // Sourced from `git@github.com:chromium/chromium.git` at `tools/origin_trials/eftest.key` on commit 49249345609d505c8bb8b0b5a42ff4b68b9e6d41.
    let content = include_bytes!("../testdata/eftest.key");
    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("should not error");
    let expected_fingerprint = make_fingerprint::<RawSHA256>(content);
    assert_eq!(combined.raw, expected_fingerprint);
    assert_eq!(combined.comment_stripped, None);
}

#[test]
fn comment_stripped_does_not_fingerprint_binary_file() {
    let content = vec![1, 2, 3, 0, 1, 2, 3];
    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("should not error");
    assert_eq!(combined.comment_stripped, None);
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

    let combined = fingerprint_stream(&mut Cursor::new(content)).expect("should not error");
    let expected = String::from("44fc8f68ab633c7ca0240a66e4ff038c0f2412fe69d14b6f052556edaa1b9160");
    assert_eq!(
        combined.comment_stripped.map(|fp| fp.to_string()),
        Some(expected)
    );
}

#[test]
fn evaluate_kinds() {
    let mut evaluated = kinds_evaluated();
    assert!(!would_evaluate_new_kinds(&evaluated));

    evaluated.remove(&SerializedKind::new(CommentStrippedSHA256.to_string()));
    assert!(would_evaluate_new_kinds(&evaluated));

    let mut evaluated = kinds_evaluated();
    evaluated.insert(SerializedKind::new("some other kind"));
    assert!(!would_evaluate_new_kinds(&evaluated));
}
