use std::io::Read;

/// The result of checking a file for whether it is binary.
///
/// The read bytes are buffered in this structure so that they can be re-read
/// by the actual fingerprint implementation.
/// ```ignore
/// let content = String::from("test_").repeat(2000);
/// let mut reader = std::io::Cursor::new(&content);
/// let Check { read, .. } = Check::content(&mut reader).expect("read");
///
/// let mut read_content = String::new();
/// std::io::Cursor::new(read).chain(reader).expect("read");
/// assert_eq!(content, read_content)
/// ```
#[derive(Clone)]
pub struct Check {
    pub read: Vec<u8>,
    pub is_binary: bool,
}

impl Check {
    /// Inspect the file to determine if it is binary.
    ///
    /// Uses the same method as git: "is there a zero byte in the first 8000 bytes of the file"
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all, ret)]
    pub fn content(stream: impl Read) -> Result<Check, std::io::Error> {
        let mut buf = Vec::with_capacity(8000);
        stream.take(buf.capacity() as u64).read_to_end(&mut buf)?;
        let is_binary = buf.contains(&0);
        Ok(Check {
            read: buf,
            is_binary,
        })
    }
}

impl std::fmt::Debug for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Check")
            .field("read", &self.read.len())
            .field("is_binary", &self.is_binary)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn chains() {
        // Generate more than 8kb of text.
        let raw_content = String::from("test_").repeat(2000);

        let mut reader = Cursor::new(&raw_content);
        let Check { read, .. } = Check::content(&mut reader).expect("read");

        // Validate that after reading the start of the stream,
        // the entire stream can be reconstructed.
        let mut read_content = String::new();
        Cursor::new(read)
            .chain(reader)
            .read_to_string(&mut read_content)
            .expect("read");

        assert_eq!(raw_content, read_content);
    }

    #[test]
    fn reports_binary() {
        let content = String::from("test_").repeat(100);
        let mut reader = Cursor::new(&content);
        let check = Check::content(&mut reader).expect("read");
        assert_eq!(false, check.is_binary);

        let content = vec![0; 100];
        let mut reader = Cursor::new(&content);
        let check = Check::content(&mut reader).expect("read");
        assert_eq!(true, check.is_binary);

        let content = {
            let mut content = Vec::<u8>::new();
            content.extend(String::from("test_").as_bytes());
            content.push(0);
            content.extend(String::from("_test").as_bytes());
            content
        };
        let mut reader = Cursor::new(&content);
        let check = Check::content(&mut reader).expect("read");
        assert_eq!(true, check.is_binary);
    }
}
