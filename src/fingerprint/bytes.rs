use std::io::{BufRead, Cursor, Read as _, Write};

use sha2::{Digest, Sha256};

use crate::{Error, Fingerprint, Kind};

use super::{binary, text};

/// Fingerprint the file as a raw chunk of bytes.
pub fn raw<R: BufRead>(stream: &mut R) -> Result<Fingerprint, Error> {
    // Read the start of the stream, and decide whether to treat the rest of the stream as binary based on that.
    let binary::Check { is_binary, read } = binary::Check::content(stream)?;

    // Chain the part of the stream already read to evaluate binary along with the rest of the stream.
    let mut stream = Cursor::new(read).chain(stream);
    let mut hasher = Sha256::new();
    if is_binary {
        content(&mut stream, &mut hasher)?;
    } else {
        text::content(&mut stream, &mut hasher)?;
    }

    Fingerprint::from_digest(Kind::RAW_SHA256, hasher)
}

/// Reads the exact contents of a binary file without modification.
pub fn content(stream: &mut impl BufRead, w: &mut impl Write) -> Result<(), Error> {
    std::io::copy(stream, w)?;
    Ok(())
}
