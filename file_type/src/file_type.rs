use crate::format::{FileFormat, RelationshipType};
use crate::{file_types, Result};
use std::cmp::Ordering;
use std::io::{Read, Seek};
use std::path::Path;

/// A file type.  The file type is determined by examining the file or bytes against known file
/// signatures and file extensions.
///
/// Additional information on PRONOM file types can be found at
/// [The National Archives](https://www.nationalarchives.gov.uk/pronom/)
///
/// # Example
///
/// Detect a Java class file from bytes:
/// ```
/// use file_type::FileType;
///
/// let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
/// assert_eq!(file_type.name(), "Java Class File");
/// assert_eq!(file_type.media_types(), Vec::<String>::new());
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
///     assert_eq!(file_type.id(), "fmt/11");
///     assert_eq!(file_type.name(), "Portable Network Graphics");
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
/// assert_eq!(file_type.id(), "fmt/11");
/// assert_eq!(file_type.name(), "Portable Network Graphics");
/// assert_eq!(file_type.extensions(), vec!["png"]);
/// assert_eq!(file_type.media_types(), vec!["image/png"]);
/// ```
#[derive(Clone, Debug)]
pub struct FileType {
    file_format: FileFormat,
}

impl FileType {
    /// Create a new `FileType` from a `FileFormat`.
    pub(crate) fn new(file_format: FileFormat) -> Self {
        FileType { file_format }
    }

    /// Get the file type identifier.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.id(), "fmt/11");
    /// ```
    #[must_use]
    pub fn id(&self) -> &str {
        self.file_format.puid()
    }

    /// Get the human-readable name of the file type
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.name(), "Portable Network Graphics");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        self.file_format.name()
    }

    /// Get the file type extensions
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// ```
    #[must_use]
    pub fn extensions(&self) -> Vec<&str> {
        self.file_format.extensions()
    }

    /// Get the [Media Type](https://www.rfc-editor.org/rfc/rfc2046.html)
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    #[must_use]
    pub fn media_types(&self) -> Vec<&str> {
        self.file_format.media_types()
    }

    /// Get the detailed file format information for this file type.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.file_format().version(), "1.0");
    /// ```
    #[must_use]
    pub fn file_format(&self) -> &FileFormat {
        &self.file_format
    }

    /// Get the file type for an identifier.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_id("fmt/11").expect("file type not found");
    /// assert_eq!(file_type.name(), "Portable Network Graphics");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    #[must_use]
    pub fn from_id<S: AsRef<str>>(id: S) -> Option<&'static Self> {
        file_types::from_id(id)
    }

    /// Get the file types for a given extension.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_extension("sqlite3");
    /// assert_eq!(1, file_types.len());
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.name(), "SQLite Database File Format");
    /// assert_eq!(file_type.media_types(), vec!["application/x-sqlite3"]);
    /// ```
    #[must_use]
    pub fn from_extension<S: AsRef<str>>(extension: S) -> &'static Vec<&'static Self> {
        file_types::from_extension(extension)
    }

    /// Get the file types for a given media type.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_types = FileType::from_media_type("text/markdown");
    /// assert_eq!(1, file_types.len());
    /// let file_type = file_types.first().expect("file format");
    /// assert_eq!(file_type.name(), "Markdown");
    /// assert_eq!(file_type.extensions(), vec!["md", "markdown"]);
    /// ```
    #[must_use]
    pub fn from_media_type<S: AsRef<str>>(media_type: S) -> &'static Vec<&'static Self> {
        file_types::from_media_type(media_type)
    }

    /// Attempt to determine the `FileType` from a byte slice.
    ///
    /// # Example
    /// ```
    /// use file_type::FileType;
    ///
    /// let file_type = FileType::from_bytes(b"\xCA\xFE\xBA\xBE");
    /// assert_eq!(file_type.name(), "Java Class File");
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
    ///     assert_eq!(file_type.id(), "x-fmt/415");
    ///     assert_eq!(file_type.name(), "Java Class File");
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
    ///     assert_eq!(file_type.id(), "fmt/11");
    ///     assert_eq!(file_type.name(), "Portable Network Graphics");
    ///     assert_eq!(file_type.extensions(), vec!["png"]);
    ///     assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// }
    /// ```
    #[cfg(feature = "tokio")]
    pub async fn try_from_file<P: AsRef<Path>>(path: P) -> Result<&'static Self> {
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
    /// assert_eq!(file_type.id(), "x-fmt/415");
    /// assert_eq!(file_type.name(), "Java Class File");
    /// assert_eq!(file_type.extensions(), vec!["class"]);
    /// assert_eq!(file_type.media_types(), Vec::<String>::new());
    /// ```
    pub fn try_from_reader_sync<R: Read>(reader: R) -> Result<&'static Self> {
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
    /// assert_eq!(file_type.id(), "fmt/11");
    /// assert_eq!(file_type.name(), "Portable Network Graphics");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    pub fn try_from_file_sync<P: AsRef<Path>>(path: P) -> Result<&'static Self> {
        file_types::try_from_file_sync(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
    const TEST_FILE_NAME: &str = "fmt-11-signature-id-58.png";

    fn test_file_path() -> PathBuf {
        let path = format!("{CRATE_DIR}/../testdata/pronom/{TEST_FILE_NAME}");
        PathBuf::from(path)
    }

    #[test]
    fn test_from_id() {
        let file_type = FileType::from_id("fmt/11").expect("file type not found");
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.extensions(), vec!["png"]);
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.file_format().id(), 664);
    }

    #[test]
    fn test_from_id_not_found() {
        let file_type = FileType::from_id("fmt/0");
        assert!(file_type.is_none());
    }

    #[test]
    fn test_from_extension() {
        let file_types = FileType::from_extension("sqlite3");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "fmt/729");
        assert_eq!(file_type.name(), "SQLite Database File Format");
        assert_eq!(file_type.media_types(), vec!["application/x-sqlite3"]);
        assert_eq!(
            file_type.extensions(),
            vec!["sqlite", "db", "db3", "sqlite3"]
        );
    }

    #[test]
    fn test_from_extension_not_found() {
        let file_types = FileType::from_extension("foo");
        assert_eq!(0, file_types.len());
    }

    #[test]
    fn test_from_media_type() {
        let file_types = FileType::from_media_type("text/markdown");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "fmt/1149");
        assert_eq!(file_type.name(), "Markdown");
        assert_eq!(file_type.extensions(), vec!["md", "markdown"]);
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
        assert_eq!(file_type.id(), "default/1");
        assert_eq!(file_type.name(), "Binary");
        assert_eq!(file_type.extensions(), Vec::<String>::new());
        assert_eq!(file_type.media_types(), vec!["application/octet-stream"]);
    }

    #[test]
    fn test_from_bytes_binary_default() {
        let value = b"\x00\x01\x02\x03";
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), "default/1");
        assert_eq!(file_type.name(), "Binary");
        assert_eq!(file_type.extensions(), Vec::<String>::new());
        assert_eq!(file_type.media_types(), vec!["application/octet-stream"]);
    }

    #[test]
    fn test_from_bytes_text_default() {
        let value = b"hello, world\n";
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), "default/2");
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
        assert_eq!(file_type.id(), "x-fmt/415");
        assert_eq!(file_type.name(), "Java Class File");
        assert_eq!(file_type.extensions(), vec!["class"]);
        assert_eq!(file_type.media_types(), Vec::<String>::new());
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file(file_path).await?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.extensions(), vec!["png"]);
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> Result<()> {
        let bytes = b"\xCA\xFE\xBA\xBE";
        let reader = std::io::BufReader::new(&bytes[..]);
        let file_type = FileType::try_from_reader_sync(reader)?;
        assert_eq!(file_type.id(), "x-fmt/415");
        assert_eq!(file_type.name(), "Java Class File");
        assert_eq!(file_type.extensions(), vec!["class"]);
        assert_eq!(file_type.media_types(), Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file_sync(file_path)?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
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
        assert_eq!(file_type.id(), "fmt/1098");
        assert_eq!(file_type.name(), "XZ File Format");
        assert_eq!(file_type.extensions(), vec!["xz"]);
        assert_eq!(file_type.media_types(), Vec::<String>::new());
    }
}
