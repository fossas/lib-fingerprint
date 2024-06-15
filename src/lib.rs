#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Cursor, Seek},
    path::Path,
    thread::ScopedJoinHandle,
};

use getset::Getters;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator, VariantNames};
use tap::Pipe;
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
    IO(#[from] std::io::Error),

    /// An error while extracting the named file from an archive.
    /// This error may be retried, but if it fails multiple times it's generally not recoverable.
    #[error("i/o error while fingerprinting '{file}': {err}")]
    Archive {
        /// The path of the file inside the archive.
        file: String,

        /// The error when fingerprinting the file.
        #[source]
        err: std::io::Error,
    },
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
/// - Update this library with the new kind(s), which is the ideal.
/// - Implement the new kind manually in your application, which is acceptable if the kind is a one-off.
///
/// ## Deserializing
///
/// When deserializing this type, the deserialized value must match a known kind;
/// otherwise a deserialization error is returned.
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Serialize,
    Display,
    AsRefStr,
    EnumIter,
    VariantNames,
)]
#[non_exhaustive]
pub enum Kind {
    /// Represents a fingerprint derived by hashing the raw contents of a file with the SHA256 algorithm.
    ///
    /// This is the default kind of fingerprint, and the kind of fingerprint with the maximal comparison signal,
    /// as the raw SHA256 hash of two files matching indicates that the two files are exactly the same content.
    /// It's also the fingerprint kind that works for literally all kinds of files, whereas other fingerprint kinds
    /// generally require specific circumstances: `CommentStrippedSHA256` requires that the file is text, and
    /// hypothetical future fingerprint kinds such as something based on an AST would require that the file is source code.
    #[strum(serialize = "sha_256")]
    RawSha256,

    /// Represents a fingerprint derived by hashing the contents of a file with the SHA256 algorithm
    /// after performing basic C-style comment stripping.
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
    #[strum(serialize = "comment_stripped:sha_256")]
    CommentStrippedSha256,

    /// Represents a fingerprint derived by hashing the raw contents of a JAR file with the SHA256 algorithm
    /// in a platform-independent manner.
    ///
    /// Specifically:
    /// - All files inside the JAR file are sorted by their names alphanumerically.
    /// - All the contents of these files are then hashed using SHA256.
    /// - If the contents of the files are text, `\r\n` sequences are converted to `\n`.
    #[strum(serialize = "v1.raw.jar")]
    JarRawV1,

    /// Represents a fingerprint derived by hashing the raw contents of a JAR file in the same manner
    /// as Maven Central. The idea is that such fingerprints can then be looked up via the
    /// Maven Central REST API as a fallback to our own indexing.
    ///
    /// Specifically:
    /// - The content of the JAR file is hashed as-is using the sha1 algorithm.
    #[strum(serialize = "v1.mavencentral.jar")]
    JarMavenCentralV1,

    /// Represents a fingerprint derived by hashing the raw contents of a JAR file with the SHA256 algorithm
    /// in a platform-independent manner.
    ///
    /// Specifically:
    /// - All files inside the JAR file are sorted by their names alphanumerically.
    /// - All the contents of these files are then hashed using SHA256.
    /// - If the contents of the files are text, `\r\n` sequences are converted to `\n`.
    #[strum(serialize = "v1.class.jar")]
    JarClassV1,
}

impl Kind {
    /// View the label as its underlying bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        for kind in Self::iter() {
            if kind.as_ref() == input {
                return Ok(kind);
            }
        }

        // let kinds = Self::iter().collect::<Vec<_>>();
        Err(serde::de::Error::custom(format!(
            "'{input}' is not a supported kind (supported: {:?})",
            Self::VARIANTS
        )))
    }
}

/// An array of bytes representing a fingerprint's content.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Content(Vec<u8>);

impl Content {
    /// Create a new instance from raw bytes.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// Create a new instance from a hash digest.
    pub fn from_digest<D: Digest>(digest: D) -> Self {
        Self(digest.finalize().as_slice().to_vec())
    }

    /// Create a new instance by hashing the provided bytes using SHA256.
    pub fn hash_sha256(content: impl AsRef<[u8]>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        Self::from_digest(hasher)
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
    #[tracing::instrument(level = tracing::Level::DEBUG, skip(stream), ret)]
    pub fn from_stream(
        kind: Kind,
        stream: impl BufRead + Seek,
    ) -> Result<Option<Fingerprint>, Error> {
        match kind {
            Kind::RawSha256 => fingerprint::bytes::raw(stream).map(Some),
            Kind::CommentStrippedSha256 => fingerprint::text::comment_stripped(stream),
            Kind::JarRawV1 => fingerprint::jar::raw(stream),
            Kind::JarClassV1 => fingerprint::jar::class(stream),
            Kind::JarMavenCentralV1 => fingerprint::jar::maven_central(stream),
        }
    }

    /// Given a desired [`Kind`], fingerprint the content of the specified file
    /// with the fingerprinter for that kind.
    ///
    /// If the fingerprinter for this [`Kind`] doesn't support the provided content,
    /// returns `Ok(None)`.
    #[tracing::instrument(level = tracing::Level::DEBUG, ret)]
    pub fn from_file(kind: Kind, path: &Path) -> Result<Option<Fingerprint>, Error> {
        let mut file = BufReader::new(File::open(path)?);
        Self::from_stream(kind, &mut file)
    }

    /// Given a desired [`Kind`], fingerprint the content of the provided buffer
    /// with the fingerprinter for that kind.
    ///
    /// If the fingerprinter for this [`Kind`] doesn't support the provided content,
    /// returns `Ok(None)`.
    #[tracing::instrument(level = tracing::Level::DEBUG, fields(buf = %buf.as_ref().len()), ret)]
    pub fn from_buffer(kind: Kind, buf: impl AsRef<[u8]>) -> Result<Option<Fingerprint>, Error> {
        let mut content = Cursor::new(buf);
        Self::from_stream(kind, &mut content)
    }

    /// Create a new instance from a digest.
    pub fn from_digest<D: Digest>(kind: Kind, digest: D) -> Self {
        let content = Content::from_digest(digest);
        Fingerprint::new(kind, content)
    }
}

impl UniqueHash for Fingerprint {
    /// Create a new hash from a fingerprint kind and a fingerprint
    fn unique_hash(&self) -> Vec<u8> {
        let mut bs = Kind::RawSha256.as_bytes().to_vec();
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
    /// Fingerprint the provided stream with all fingerprint [`Kind`]s.
    ///
    /// Note: this forces fingerprinting to be performed serially
    /// since the stream has to be seeked backwards for each fingerprinter;
    /// if this is not desired consider [`Combined::from_file`] or [`Combined::from_buffer`].
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
    pub fn from_stream(mut stream: impl BufRead + Seek) -> Result<Self, Error> {
        let mut fingerprints = Vec::new();
        for (i, kind) in Kind::iter().enumerate() {
            if i > 0 {
                stream.seek(std::io::SeekFrom::Start(0))?;
            }

            if let Some(fp) = Fingerprint::from_stream(kind, &mut stream)? {
                fingerprints.push(fp);
            }
        }
        Ok(Combined::from(fingerprints))
    }

    /// Fingerprint the provided file with all fingerprint [`Kind`]s.
    ///
    /// Note: this opens the file multiple times, once for each kind of fingerprint,
    /// then runs each fingerprinter in its own thread.
    /// If this is not desired consider [`Combined::from_stream`] or [`Combined::from_buffer`].
    #[tracing::instrument(level = tracing::Level::DEBUG, ret)]
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        std::thread::scope(|scope| {
            let handles = Kind::iter()
                .map(|kind| scope.spawn(move || Fingerprint::from_file(kind, path)))
                .collect::<Vec<_>>();

            match collapse_handles(handles) {
                Ok(fps) => fps.into_iter().flatten().pipe(Combined::from).pipe(Ok),
                Err(err) => Err(err),
            }
        })
    }

    /// Fingerprint the provided buffer with all fingerprint [`Kind`]s.
    ///
    /// ## Errors
    ///
    /// As of writing this comment, the only [`Error`] variants are IO-related,
    /// which are not possible to encounter when reading fully buffered content,
    /// so theoretically this error can be ignored or safely unwrapped.
    ///
    /// This library returns the error anyway so that if it introduces more kinds
    /// of errors in the future it isn't a breaking change.
    #[tracing::instrument(level = tracing::Level::DEBUG, fields(buf = %buf.as_ref().len()), ret)]
    pub fn from_buffer(buf: impl AsRef<[u8]>) -> Result<Self, Error> {
        Kind::iter()
            .map(|kind| Fingerprint::from_buffer(kind, buf.as_ref()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .pipe(Combined::from)
            .pipe(Ok)
    }

    /// Create a new instance from a single fingerprint.
    pub fn single(fp: impl Into<Fingerprint>) -> Self {
        Self::from([fp])
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

    /// Convert into a hashmap of constituent parts.
    pub fn into_inner(self) -> HashMap<Kind, Content> {
        self.0
    }
}

impl<I: IntoIterator<Item = F>, F: Into<Fingerprint>> From<I> for Combined {
    fn from(value: I) -> Self {
        Self(
            value
                .into_iter()
                .map(|fp| fp.into())
                .map(|Fingerprint { kind, content }| (kind, content))
                .collect(),
        )
    }
}

fn collapse_handles<T, E>(handles: Vec<ScopedJoinHandle<'_, Result<T, E>>>) -> Result<Vec<T>, E> {
    let mut collected = Vec::new();
    for handle in handles {
        match handle.join() {
            Err(err) => std::panic::resume_unwind(err),
            Ok(operation) => match operation {
                Ok(inner) => collected.push(inner),
                Err(err) => return Err(err),
            },
        }
    }
    Ok(collected)
}
