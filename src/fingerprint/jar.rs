use std::io::{BufRead, Read, Seek};

use sha1::Sha1;
use sha2::{Digest, Sha256};
use tap::Pipe;
use tracing::warn;
use zip::read::ZipFile;

use crate::{Content, Error, Fingerprint, Kind};

/// Fingerprint all files inside a java archive (a JAR).
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn raw(stream: impl BufRead + Seek) -> Result<Option<Fingerprint>, Error> {
    match files(Kind::JarRawV1, stream, |_| true) {
        Ok(fp) => Ok(fp),
        Err(err) => match err {
            zip::result::ZipError::Io(err) => return Err(Error::IO(err)),
            err => {
                warn!(?err, "jar may be malformed");
                return Ok(None);
            }
        },
    }
}

/// Fingerprint the java archive the same way as Maven Central.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn maven_central(mut stream: impl BufRead + Seek) -> Result<Option<Fingerprint>, Error> {
    let mut hasher = Sha1::new();
    std::io::copy(&mut stream, &mut hasher)?;
    let content = Content::from_digest(hasher);
    Ok(Some(Fingerprint::new(Kind::JarMavenCentralV1, content)))
}

/// Fingerprint class files inside a java archive (a JAR).
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
pub fn class(stream: impl BufRead + Seek) -> Result<Option<Fingerprint>, Error> {
    match files(Kind::JarClassV1, stream, is_class_file) {
        Ok(fp) => Ok(fp),
        Err(err) => match err {
            zip::result::ZipError::Io(err) => return Err(Error::IO(err)),
            err => {
                warn!(?err, "jar may be malformed");
                return Ok(None);
            }
        },
    }
}

#[tracing::instrument(level = tracing::Level::DEBUG, fields(entry = ?entry.name()), ret)]
fn is_class_file<R: Read>(entry: &ZipFile<'_, R>) -> bool {
    // If this becomes a problem in the future we can use the magic number instead:
    // https://en.wikipedia.org/wiki/Java_class_file#Magic_Number
    entry.name().ends_with(".class")
}

// Due to https://github.com/zip-rs/zip2/issues/178 we are required to use the `Seek` bound.
// tl;dr: older Java versions create JARs that do not work in streaming mode.
#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, fields(kind), ret)]
fn files<R: BufRead + Seek>(
    kind: Kind,
    stream: R,
    include: impl Fn(&ZipFile<'_, R>) -> bool,
) -> zip::result::ZipResult<Option<Fingerprint>> {
    let mut files = Vec::new();
    let mut archive = zip::ZipArchive::new(stream)?;

    for index in 0..archive.len() {
        let entry = archive.by_index(index)?;
        if !include(&entry) {
            continue;
        }

        let pair = file(entry)?;
        files.push(pair);
    }

    combine(kind, files).pipe(Some).pipe(Ok)
}

#[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
fn file<R: Read>(mut entry: ZipFile<'_, R>) -> zip::result::ZipResult<(String, Content)> {
    let name = entry.name().to_string();
    let mut hasher = Sha256::new();
    std::io::copy(&mut entry, &mut hasher)?;

    let content = Content::from_digest(hasher);
    Ok((name, content))
}

#[tracing::instrument(level = tracing::Level::DEBUG, fields(jars = %jars.len()), ret)]
fn combine(kind: Kind, mut jars: Vec<(String, Content)>) -> Fingerprint {
    jars.sort_unstable_by(|(a, _), (b, _)| alphanumeric_sort::compare_str(a, b));
    jars.into_iter()
        .fold(Sha256::new(), |mut hasher, (_, content)| {
            hasher.update(content.as_bytes());
            hasher
        })
        .pipe(|digest| Fingerprint::from_digest(kind, digest))
}
