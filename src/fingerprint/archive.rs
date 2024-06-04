use std::io::BufRead;

use crate::{Error, Fingerprint};

/// Fingerprint all files inside a java archive (a JAR).
pub fn jar_raw<R: BufRead>(stream: &mut R) -> Result<Option<Fingerprint>, Error> {
    todo!()
}

/// Fingerprint class files inside a java archive (a JAR).
pub fn jar_class<R: BufRead>(stream: &mut R) -> Result<Option<Fingerprint>, Error> {
    todo!()
}
