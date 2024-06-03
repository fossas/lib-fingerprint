//! Utilities for streaming byte oriented operations.

use std::{io, iter::Peekable};

const LF_CHAR: u8 = b'\n';
const CR_CHAR: u8 = b'\r';

/// Convenience trait representing an iterator of a byte stream (as returned from `Read::bytes`).
/// Automatically implemented.
pub(crate) trait ByteIterator: Iterator<Item = io::Result<u8>> {}
impl<I> ByteIterator for I where I: Iterator<Item = io::Result<u8>> {}

/// Implements the ability to drop `\r\n` byte pairs from a stream, converting each instance to a single `\n`.
pub(crate) struct CRLFToLF<I: ByteIterator> {
    iter: Peekable<I>,
}

impl<I> Iterator for CRLFToLF<I>
where
    I: ByteIterator,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the read byte is `\r`, check the byte after that (the "upcoming" byte):
        // - If this is the end of the stream, just drop the `\r`.
        // - If the upcoming byte is `\n`, drop the currently read `\r` by re-running.
        // - If the upcoming byte is neither of those things, emit the `\r`.
        //
        // The goal here is to replicate the following Haskell reference:
        // https://github.com/fossas/fossa-cli/blob/bde67a0157b8b8b8472056bea843a30d4e495271/src/App/Fossa/VSI/Fingerprint.hs#L88-L89
        // which effectively splits the stream into chunks on `\n` boundaries, then from each chunk trims the final `\r` if it exists.
        // A `\r` immediately proceeding the end of a stream is dropped because that would have been a final chunk in the Haskell version,
        // which then would have had its trailing `\r` dropped.
        match self.iter.next()? {
            Ok(byte) => {
                if byte == CR_CHAR {
                    if let Ok(next) = self.iter.peek()? {
                        if next == &LF_CHAR {
                            return self.next();
                        }
                    }
                }
                Some(Ok(byte))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub(crate) trait ConvertCRLFToLF {
    /// Drops `\r\n` byte pairs from a stream, converting each instance to a single `\n`.
    fn crlf_to_lf(self) -> CRLFToLF<Self>
    where
        Self: Sized,
        Self: ByteIterator;
}

impl<I> ConvertCRLFToLF for I
where
    I: ByteIterator,
{
    fn crlf_to_lf(self) -> CRLFToLF<Self> {
        CRLFToLF {
            iter: self.peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    //! Tests for internal logic.

    use std::io::{Cursor, Read};

    use super::*;

    #[test]
    fn crlf_to_lf_works() {
        let content = b"hello\r\neveryone\nin\r\nthe\nworld\r";
        let expected = b"hello\neveryone\nin\nthe\nworld".to_vec();

        let processed = Cursor::new(content)
            .bytes()
            .crlf_to_lf()
            .collect::<io::Result<Vec<u8>>>()
            .expect("should not error");

        assert_eq!(expected, processed);
    }
}
