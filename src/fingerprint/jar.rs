use std::io::BufRead;

use sha2::{Digest, Sha256};
use tap::Pipe;
use zip::read::ZipFile;

use crate::{Content, Error, Fingerprint, Kind};

/// Fingerprint all files inside a java archive (a JAR).
pub fn raw<R: BufRead>(stream: &mut R) -> Result<Option<Fingerprint>, Error> {
    fingerprint_files(Kind::JarRawV1, stream, |_| true)
}

/// Fingerprint class files inside a java archive (a JAR).
pub fn class<R: BufRead>(stream: &mut R) -> Result<Option<Fingerprint>, Error> {
    fingerprint_files(Kind::JarRawV1, stream, |file| {
        file.name().ends_with(".class")
    })
}

fn fingerprint_files<R: BufRead>(
    kind: Kind,
    stream: &mut R,
    include: impl Fn(&ZipFile<'_>) -> bool,
) -> Result<Option<Fingerprint>, Error> {
    let mut files = Vec::new();
    loop {
        match zip::read::read_zipfile_from_stream(stream) {
            Ok(None) => break,
            Ok(Some(file)) => {
                if include(&file) {
                    let pair = fingerprint_file(file)?;
                    files.push(pair);
                }
            }
            Err(err) => match err {
                zip::result::ZipError::Io(err) => return Err(Error::IO(err)),
                _ => return Ok(None),
            },
        }
    }
    combine_fingerprints(kind, files).pipe(Some).pipe(Ok)
}

fn combine_fingerprints(kind: Kind, mut jars: Vec<(String, Content)>) -> Fingerprint {
    jars.sort_unstable_by(|(a, _), (b, _)| alphanumeric_sort::compare_str(a, b));
    jars.into_iter()
        .fold(Sha256::new(), |mut hasher, (_, content)| {
            hasher.update(content.as_bytes());
            hasher
        })
        .pipe(|digest| Fingerprint::from_digest(kind, digest))
}

fn fingerprint_file(mut file: ZipFile<'_>) -> Result<(String, Content), Error> {
    let name = file.name().to_string();
    let mut hasher = Sha256::new();
    match std::io::copy(&mut file, &mut hasher) {
        Ok(_) => {
            let content = Content::from_digest(hasher);
            Ok((name, content))
        }
        Err(err) => Err(Error::Archive { file: name, err }),
    }
}
