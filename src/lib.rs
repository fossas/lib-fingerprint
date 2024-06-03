#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Seek},
    path::Path,
};

use getset::Getters;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

mod fingerprint;
mod stream;

/// Errors that may be encountered during fingerprinting.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A generic IO error occurred while reading the content to be hashed.
    /// This error may be retried, but if it fails multiple times it's generally not recoverable.
    #[error("i/o error: {0}")]
    IO(#[from] io::Error),
}

/// Fingerprint kinds MUST maintain exact implementation compatibility; once the algorithm for a given kind
/// has been created and its fingerprints have been crawled, it can't be changed. If a change is needed,
/// that has to be a new kind of fingerprint. Similarly, the text representation for a given algorithm
/// cannot change either: some services assume certain things about the fingerprints that we cannot easily change
/// (for example, Sherlock assumes all files have a `sha_256` fingerprint).
///
/// This is because fingerprints form the backbone of how VSI operates:
/// - FOSSA CLI and Azathoth create them.
/// - Sherlock and Multivac assume certain things about them.
/// - All implementations must create them in the same way and have the same assumptions.
///
/// For this reason, new kinds cannot be created outside of this library,
/// and new kinds cannot be created at runtime.
///
/// ## Sparkle compatibility
///
/// Sparkle itself supports user-created kinds, but this library is a "user";
/// in other words while _Sparkle_ supports arbitrary kinds _this library_ defines (some of) those kinds.
///
/// If you want to use other kinds, prefer to:
/// - Update this library with the new kind(s) (ideal).
/// - Implement the new kind manually in your application (acceptable if the kind is a one-off).
///
/// ## Deserializing
///
/// When deserializing this type, the deserialized value must match a known kind;
/// otherwise a deserialization error is returned.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize)]
pub struct Kind(&'static str);

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Kind {
    /// Represents a fingerprint derived by hashing the raw contents of a file with the SHA256 algorithm.
    ///
    /// This is the default kind of fingerprint, and the kind of fingerprint with the maximal comparison signal,
    /// as the raw SHA256 hash of two files matching indicates that the two files are exactly the same content.
    /// It's also the fingerprint kind that works for literally all kinds of files, whereas other fingerprint kinds
    /// generally require specific circumstances: `CommentStrippedSHA256` requires that the file is text, and
    /// hypothetical future fingerprint kinds such as something based on an AST would require that the file is source code.
    pub const RAW_SHA256: Kind = Kind("sha_256");

    /// Represents a fingerprint derived by hashing the contents of a file with the SHA256 algorithm
    /// after performing basic C-style comment stripping.
    pub const COMMENT_STRIPPED_SHA256: Kind = Kind("comment_stripped:sha_256");

    /// List all built in kinds.
    pub fn enumerate() -> impl Iterator<Item = Self> {
        [Self::RAW_SHA256, Self::COMMENT_STRIPPED_SHA256].into_iter()
    }

    /// View the kind as its underlying bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parsed = String::deserialize(deserializer)?;
        for kind in Self::enumerate() {
            if kind.0 == parsed {
                return Ok(kind);
            }
        }

        let kinds = Self::enumerate().collect::<Vec<_>>();
        Err(serde::de::Error::custom(format!(
            "kind '{parsed}' is not a supported kind (supported kinds: {kinds:?})"
        )))
    }
}

/// An array of bytes representing a fingerprint's content.
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct Content(Vec<u8>);

impl Content {
    /// Create a new instance from raw bytes.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// Create a new instance from a hash digest.
    pub fn from_digest<D: Digest>(digest: D) -> Result<Self, Error> {
        let buf = digest.finalize().as_slice().to_vec();
        Ok(Self(buf))
    }

    /// Reference the bytes inside the blob.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "fp-content-serialize-hex")]
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

#[cfg(all(
    not(feature = "fp-content-serialize-hex"),
    feature = "fp-content-serialize-base64"
))]
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::{prelude::*, Engine};
        write!(f, "{}", &BASE64_STANDARD.encode(&self.0))
    }
}

#[cfg(any(
    feature = "fp-content-serialize-hex",
    feature = "fp-content-serialize-base64"
))]
impl std::fmt::Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(all(
    not(feature = "fp-content-serialize-hex"),
    not(feature = "fp-content-serialize-base64")
))]
impl std::fmt::Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Content").field(&self.0).finish()
    }
}

#[cfg(feature = "fp-content-serialize-hex")]
impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

#[cfg(feature = "fp-content-serialize-hex")]
impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let b = hex::decode(s).map_err(serde::de::Error::custom)?;
        Ok(Self(b))
    }
}

#[cfg(all(
    not(feature = "fp-content-serialize-hex"),
    feature = "fp-content-serialize-base64"
))]
impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use base64::{prelude::*, Engine};
        serializer.serialize_str(&BASE64_STANDARD.encode(&self.0))
    }
}

#[cfg(all(
    not(feature = "fp-content-serialize-hex"),
    feature = "fp-content-serialize-base64"
))]
impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use base64::{prelude::*, Engine};
        let s = String::deserialize(deserializer)?;
        let b = BASE64_STANDARD
            .decode(s)
            .map_err(serde::de::Error::custom)?;
        Ok(Self(b))
    }
}

/// Use the `kind` and `content` to derive a hash per unique fingerprint.
pub trait UniqueHash {
    /// Uniquely identify the fingerprint as a hash of its kind and content.
    fn unique_hash(&self) -> Vec<u8>;
}

/// An opaque, deterministic value for the file's contents.
/// If two fingerprints are the same, the contents of the files used to create the fingerprints are the same.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Getters)]
#[getset(get = "pub")]
pub struct Fingerprint {
    /// The method used to generate a fingerprint.
    kind: Kind,

    /// The deterministic identifier for the fingerprint.
    content: Content,
}

impl Fingerprint {
    /// Create a new instance.
    pub fn new(kind: impl Into<Kind>, content: impl Into<Content>) -> Self {
        Self {
            kind: kind.into(),
            content: content.into(),
        }
    }

    /// Given a desired [`Kind`], fingerprint the content of the provided stream
    /// with the fingerprinter for that kind.
    ///
    /// If the fingerprinter for this [`Kind`] doesn't support the provided content,
    /// returns `Ok(None)`.
    ///
    /// ## Panic
    ///
    /// If a [`Kind`] is provided that is not accounted for in this function, it panicks.
    /// This should not happen in normal program execution since kinds cannot be created
    /// outside of this library, so this situation is automatically a bug in this library.
    pub fn generate_stream(
        kind: Kind,
        stream: &mut impl BufRead,
    ) -> Result<Option<Fingerprint>, Error> {
        match kind {
            Kind::RAW_SHA256 => fingerprint::raw(stream).map(Some),
            Kind::COMMENT_STRIPPED_SHA256 => fingerprint::comment_stripped(stream),
            unknown => panic!("unsupported kind: {unknown}"),
        }
    }

    /// Given a desired [`Kind`], fingerprint the content of the specified file
    /// with the fingerprinter for that kind.
    ///
    /// If the fingerprinter for this [`Kind`] doesn't support the provided content,
    /// returns `Ok(None)`.
    ///
    /// ## Panic
    ///
    /// If a [`Kind`] is provided that is not accounted for in this function, it panicks.
    /// This should not happen in normal program execution since kinds cannot be created
    /// outside of this library, so this situation is automatically a bug in this library.
    pub fn generate_file(kind: Kind, path: &Path) -> Result<Option<Fingerprint>, Error> {
        let mut file = BufReader::new(File::open(path)?);
        Self::generate_stream(kind, &mut file)
    }

    /// Given a desired [`Kind`], fingerprint the content of the provided buffer
    /// with the fingerprinter for that kind.
    ///
    /// If the fingerprinter for this [`Kind`] doesn't support the provided content,
    /// returns `Ok(None)`.
    ///
    /// ## Panic
    ///
    /// If a [`Kind`] is provided that is not accounted for in this function, it panicks.
    /// This should not happen in normal program execution since kinds cannot be created
    /// outside of this library, so this situation is automatically a bug in this library.
    pub fn generate(kind: Kind, buf: impl AsRef<[u8]>) -> Result<Option<Fingerprint>, Error> {
        let mut content = Cursor::new(buf);
        Self::generate_stream(kind, &mut content)
    }

    /// Create a new instance from a digest.
    pub fn from_digest<D: Digest>(kind: Kind, digest: D) -> Result<Self, Error> {
        let content = Content::from_digest(digest)?;
        Ok(Fingerprint::new(kind, content))
    }
}

impl UniqueHash for Fingerprint {
    /// Create a new hash from a fingerprint kind and a fingerprint
    fn unique_hash(&self) -> Vec<u8> {
        let mut bs = Kind::RAW_SHA256.as_bytes().to_vec();
        bs.extend_from_slice(self.content.as_bytes());
        Sha256::digest(&bs).to_vec()
    }
}

impl std::fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.kind, self.content)
    }
}

impl From<&Fingerprint> for Fingerprint {
    fn from(value: &Fingerprint) -> Self {
        value.to_owned()
    }
}

impl<K: Into<Kind>, C: Into<Content>> From<(K, C)> for Fingerprint {
    fn from((k, c): (K, C)) -> Self {
        Self::new(k, c)
    }
}

/// A collection of fingerprints on some given content.
///
/// For example, this means that if [`Combined`] is created over a binary file, [`CommentStrippedSHA256`] is not
/// in the resulting data structure, because that kind of fingerprint requires UTF8 encoded text content to run.
#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Combined(HashMap<Kind, Content>);

impl Combined {
    /// Create a new instance from an iterator of fingerprints.
    pub fn new<I: IntoIterator<Item = impl Into<Fingerprint>>>(iter: I) -> Self {
        Self(
            iter.into_iter()
                .map(|fp| fp.into())
                .map(|Fingerprint { kind, content }| (kind, content))
                .collect(),
        )
    }

    /// Create a new instance from a single fingerprint.
    pub fn single(fp: impl Into<Fingerprint>) -> Self {
        Self::new([fp.into()])
    }

    /// Get the corresponding fingerprint for the provided [`Kind`],
    /// if it exists.
    pub fn get(&self, kind: Kind) -> Option<&Content> {
        self.0.get(&kind)
    }

    /// Iterate through the fingerprints in the collection.
    pub fn iter(&self) -> impl Iterator<Item = (&Kind, &Content)> {
        self.0.iter()
    }
}

impl IntoIterator for Combined {
    type Item = Fingerprint;

    type IntoIter = std::iter::Map<
        std::collections::hash_map::IntoIter<Kind, Content>,
        fn((Kind, Content)) -> Fingerprint,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(Fingerprint::from)
    }
}

impl<I: IntoIterator<Item = Option<Fingerprint>>> From<I> for Combined {
    fn from(value: I) -> Self {
        Self(
            value
                .into_iter()
                .filter_map(|fp| {
                    let Fingerprint { kind, content } = fp?;
                    Some((kind, content))
                })
                .collect(),
        )
    }
}

/// Fingerprint the provided file with all fingerprint [`Kind`]s.
pub fn fingerprint_file(path: &Path) -> Result<Combined, Error> {
    let mut file = BufReader::new(File::open(path)?);
    fingerprint_stream(&mut file)
}

/// Fingerprint the provided stream (typically a file handle) with all fingerprint [`Kind`]s.
pub fn fingerprint_stream<R: BufRead + Seek>(stream: &mut R) -> Result<Combined, Error> {
    let raw = Fingerprint::generate_stream(Kind::RAW_SHA256, stream)?;
    stream.seek(io::SeekFrom::Start(0))?;
    let cs = Fingerprint::generate_stream(Kind::COMMENT_STRIPPED_SHA256, stream)?;
    Ok(Combined::from([raw, cs]))
}

/// Fingerprint the provided buffer with all fingerprint [`Kind`]s.
///
/// ## Errors
///
/// As of writing this comment, the only [`Error`] variant is [`Error::IO`],
/// which is not possible to encounter when reading fully buffered content,
/// so theoretically this error can be ignored or safely unwrapped.
///
/// This library returns the error anyway so that if we introduce more kinds
/// of errors in the future it isn't a breaking change.
pub fn fingerprint(buf: impl AsRef<[u8]>) -> Result<Combined, Error> {
    let mut content = Cursor::new(buf);
    fingerprint_stream(&mut content)
}
