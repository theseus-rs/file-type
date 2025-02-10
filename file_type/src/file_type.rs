use crate::format::{FileFormat, SourceType};
use crate::{file_types, Result};
use core::cmp::Ordering;

/// A file type.  The file type is determined by examining the file or bytes against known file
/// signatures and file extensions.
///
/// # Example
///
/// Detect a Java class file from bytes:
/// ```
/// use file_type::FileType;
///
/// let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
/// assert_eq!(file_type.extensions(), vec!["class"]);
/// ```
///
/// Detect the file type from a file:
/// ```rust,no_run
/// use file_type::FileType;
/// use std::path::Path;
///
/// #[tokio::main]
/// async fn main() {
///     let file_path = Path::new("image.png");
///     let file_type = FileType::try_from_file(file_path).await.expect("file type not found");
///     assert_eq!(file_type.extensions(), vec!["png"]);
///     assert_eq!(file_type.media_types(), vec!["image/png"]);
/// }
/// ```
///
/// Detect the file type from a file synchronously:
/// ```no_run
/// use file_type::FileType;
/// use std::path::Path;
///
/// let file_path = Path::new("image.png");
/// let file_type = FileType::try_from_file_sync(file_path).expect("file type not found");
/// assert_eq!(file_type.extensions(), vec!["png"]);
/// assert_eq!(file_type.media_types(), vec!["image/png"]);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileType {
    pub(crate) file_format: &'static FileFormat,
}

impl FileType {
    /// Create a new `FileType` from a `FileFormat`.
    pub(crate) fn new(file_format: &'static FileFormat) -> Self {
        FileType { file_format }
    }

    /// Get the file type identifier.
    #[doc(hidden)]
    #[must_use]
    pub fn id(&self) -> usize {
        self.file_format.id
    }

    /// Get the source for this file type.
    #[doc(hidden)]
    #[must_use]
    pub fn source_type(&self) -> &SourceType {
        &self.file_format.source_type
    }

    /// Get the human-readable name of the file type
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_media_type("image/png");
    /// let file_type = file_types.first().expect("file format");
    /// assert!(file_type.name().contains("Portable Network Graphics"));
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        self.file_format.name
    }

    /// Get the file type extensions
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_media_type("image/png");
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// ```
    #[must_use]
    pub fn extensions(&self) -> &[&str] {
        self.file_format.extensions
    }

    /// Get the [Media Type](https://www.rfc-editor.org/rfc/rfc2046.html)
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_extension("png");
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    #[must_use]
    pub fn media_types(&self) -> &[&str] {
        self.file_format.media_types
    }

    /// Get the file format information for this file type.
    #[doc(hidden)]
    #[must_use]
    pub fn file_format(&self) -> &FileFormat {
        self.file_format
    }

    /// Get the file types for a given extension.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_extension("duckdb");
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.media_types(), vec!["application/vnd.duckdb.file"]);
    /// ```
    #[must_use]
    pub fn from_extension<S: AsRef<str>>(extension: S) -> &'static [&'static Self] {
        file_types::from_extension(extension)
    }

    /// Get the file types for a given media type.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_media_type("image/png");
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// ```
    #[must_use]
    pub fn from_media_type<S: AsRef<str>>(media_type: S) -> &'static [&'static Self] {
        file_types::from_media_type(media_type)
    }

    /// Attempt to determine the `FileType` from a byte slice.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
    /// assert_eq!(file_type.media_types(), Vec::<String>::new());
    /// assert_eq!(file_type.extensions(), vec!["class"]);
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> &'static Self {
        file_types::from_bytes(bytes, None)
    }

    /// Attempt to determine the `FileType` from a reader.
    ///
    /// # Errors
    /// if the file type is unknown
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    /// use tokio::io::BufReader;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bytes = b"\xCA\xFE\xBA\xBE";
    ///     let reader = BufReader::new(&bytes[..]);
    ///     let file_type = FileType::try_from_reader(reader).await.expect("file type not found");
    ///     assert_eq!(file_type.extensions(), vec!["class"]);
    ///     assert_eq!(file_type.media_types(), Vec::<String>::new());
    /// }
    /// ```
    #[cfg(feature = "tokio")]
    pub async fn try_from_reader<R>(reader: R) -> Result<&'static Self>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        file_types::try_from_reader(reader, None).await
    }

    /// Attempt to determine the `FileType` from a file.
    ///
    /// # Errors
    /// if the file type is unknown
    ///
    /// # Example
    /// ```no_run
    /// use file_type::FileType;
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let file_path = Path::new("image.png");
    ///     let file_type = FileType::try_from_file(file_path).await.expect("file type not found");
    ///     assert_eq!(file_type.extensions(), vec!["png"]);
    ///     assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// }
    /// ```
    #[cfg(feature = "tokio")]
    pub async fn try_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<&'static Self> {
        file_types::try_from_file(path).await
    }

    /// Attempt to determine the `FileType` from a reader synchronously.
    ///
    /// # Errors
    /// if the file type is unknown
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    /// use std::io::BufReader;
    ///
    /// let bytes = b"\xCA\xFE\xBA\xBE";
    /// let reader = BufReader::new(&bytes[..]);
    /// let file_type = FileType::try_from_reader_sync(reader).expect("file type not found");
    /// assert_eq!(file_type.extensions(), vec!["class"]);
    /// assert_eq!(file_type.media_types(), Vec::<String>::new());
    /// ```
    pub fn try_from_reader_sync<R: std::io::Read>(reader: R) -> Result<&'static Self> {
        file_types::try_from_reader_sync(reader, None)
    }

    /// Attempt to determine the `FileType` from a file synchronously.
    ///
    /// # Errors
    /// if the file type is unknown
    ///
    /// # Example
    /// ```no_run
    /// use file_type::FileType;
    /// use std::path::Path;
    ///
    /// let file_path = Path::new("image.png");
    /// let file_type = FileType::try_from_file_sync(file_path).expect("file type not found");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    pub fn try_from_file_sync<P: AsRef<std::path::Path>>(path: P) -> Result<&'static Self> {
        file_types::try_from_file_sync(path)
    }
}

impl Ord for FileType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_format.cmp(other.file_format)
    }
}

impl PartialOrd for FileType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.file_format.cmp(other.file_format))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::SourceType;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use std::path::PathBuf;

    const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
    const TEST_FILE_NAME: &str = "pronom-664-signature-0.png";

    fn test_file_path() -> PathBuf {
        PathBuf::from(CRATE_DIR)
            .join("..")
            .join("test_data")
            .join("pronom")
            .join(TEST_FILE_NAME)
    }

    #[cfg(feature = "custom")]
    #[test]
    fn test_from_extension() {
        let file_types = FileType::from_extension("duckdb");
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), 3);
        assert_eq!(file_type.name(), "DuckDB");
        assert_eq!(file_type.media_types(), vec!["application/vnd.duckdb.file"]);
        assert_eq!(file_type.extensions(), vec!["duckdb"]);
    }

    #[test]
    fn test_from_extension_not_found() {
        let file_types = FileType::from_extension("foo");
        assert_eq!(0, file_types.len());
    }

    #[test]
    fn test_from_media_type() {
        let file_types = FileType::from_media_type("image/png");
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.extensions(), vec!["png"]);
    }

    #[test]
    fn test_from_media_type_not_found() {
        let file_types = FileType::from_media_type("foo/bar");
        assert_eq!(0, file_types.len());
    }

    #[test]
    fn test_from_bytes_empty_default() {
        let value = Vec::new();
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), 1);
        assert_eq!(file_type.name(), "Binary");
        assert_eq!(file_type.extensions(), Vec::<String>::new());
        assert_eq!(file_type.media_types(), vec!["application/octet-stream"]);
    }

    #[test]
    fn test_from_bytes_binary_default() {
        let value = b"\x00\x01\x02\x03";
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), 1);
        assert_eq!(file_type.name(), "Binary");
        assert_eq!(file_type.extensions(), Vec::<String>::new());
        assert_eq!(file_type.media_types(), vec!["application/octet-stream"]);
    }

    #[test]
    fn test_from_bytes_text_default() {
        let value = b"hello, world\n";
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), 2);
        assert_eq!(file_type.name(), "Text");
        assert_eq!(file_type.extensions(), Vec::<String>::new());
        assert_eq!(file_type.media_types(), vec!["text/plain"]);
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_reader() -> Result<()> {
        let bytes = b"\xCA\xFE\xBA\xBE";
        let reader = tokio::io::BufReader::new(&bytes[..]);
        let file_type = FileType::try_from_reader(reader).await?;
        assert_eq!(file_type.extensions(), vec!["class"]);
        assert_eq!(file_type.media_types(), Vec::<String>::new());
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file(file_path).await?;
        assert_eq!(file_type.extensions(), vec!["png"]);
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> Result<()> {
        let bytes = b"\xCA\xFE\xBA\xBE";
        let reader = std::io::BufReader::new(&bytes[..]);
        let file_type = FileType::try_from_reader_sync(reader)?;
        assert_eq!(file_type.extensions(), vec!["class"]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file_sync(file_path)?;
        assert_eq!(file_type.extensions(), vec!["png"]);
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        Ok(())
    }

    fn large_bytes() -> Vec<u8> {
        let length = 1 << 31;
        let mut bytes = vec![0; length];
        bytes[0] = 0xFD;
        bytes[1] = 0x37;
        bytes[2] = 0x7A;
        bytes[3] = 0x58;
        bytes[4] = 0x5A;
        bytes[5] = 0x00;
        bytes[length - 2] = 0x59;
        bytes[length - 1] = 0x5A;
        bytes
    }

    #[test]
    fn test_from_bytes_large() {
        let bytes = large_bytes();
        let file_type = FileType::from_bytes(&bytes);
        #[cfg(feature = "pronom")]
        {
            assert_eq!(file_type.id(), 1907);
            assert_eq!(file_type.file_format().source_type, SourceType::Pronom);
        }
        #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
        {
            assert_eq!(file_type.id(), 162839);
            assert_eq!(file_type.file_format().source_type, SourceType::Wikidata);
        }
        assert_eq!(file_type.extensions(), vec!["xz"]);
    }
}
