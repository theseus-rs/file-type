use crate::pronom::FileFormat;
use crate::Error::UnknownFileType;
use crate::{file_types, Result};
use std::io::{Read, Seek};
use std::path::Path;
use tokio::io::{AsyncRead, AsyncSeek};
use tokio::runtime::Builder;

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
/// assert_eq!(file_type.extensions(), vec!["class".to_string()]);
/// ```
///
/// Detect text from bytes:
/// ```
/// use file_type::FileType;
///
/// let file_type = FileType::from_bytes(b"hello, world\n");
/// assert_eq!(file_type.name(), "Text");
/// assert_eq!(file_type.media_types(), vec!["text/plain".to_string()]);
/// assert_eq!(file_type.extensions(), Vec::<String>::new());
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
    #[must_use]
    pub fn id(&self) -> &str {
        self.file_format.puid()
    }

    /// Get the human-readable name of the file type
    #[must_use]
    pub fn name(&self) -> &str {
        self.file_format.name()
    }

    /// Get the [Media Type](https://www.rfc-editor.org/rfc/rfc2046.html)
    #[must_use]
    pub fn media_types(&self) -> Vec<&str> {
        self.file_format.media_types()
    }

    /// Get the file type extensions
    #[must_use]
    pub fn extensions(&self) -> Vec<&str> {
        self.file_format.extensions()
    }

    /// Get the file format
    #[must_use]
    pub fn file_format(&self) -> &FileFormat {
        &self.file_format
    }

    /// Get the file type for an identifier.
    #[must_use]
    pub fn from_id<S: AsRef<str>>(id: S) -> Option<&'static Self> {
        file_types::from_id(id)
    }

    /// Get the file types for a given extension.
    #[must_use]
    pub fn from_extension<S: AsRef<str>>(extension: S) -> &'static Vec<&'static Self> {
        file_types::from_extension(extension)
    }

    const EMPTY_MEDIA_TYPES: Vec<&'static FileType> = Vec::new();

    /// Get the file types for a given media type.
    #[must_use]
    pub fn from_media_type<S: AsRef<str>>(media_type: S) -> &'static Vec<&'static Self> {
        file_types::from_media_type(media_type)
    }

    /// Attempt to determine the `FileType` from a byte slice.
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> &'static Self {
        file_types::from_bytes(bytes, None)
    }

    /// Attempt to determine the `FileType` from a reader.
    ///
    /// # Errors
    /// if the file type is unknown
    pub async fn try_from_reader<R>(reader: R) -> Result<&'static Self>
    where
        R: AsyncRead + AsyncSeek + Unpin,
    {
        file_types::try_from_reader(reader, None).await
    }

    /// Attempt to determine the `FileType` from a file.
    ///
    /// # Errors
    /// if the file type is unknown
    pub async fn try_from_file<P: AsRef<Path>>(path: P) -> Result<&'static Self> {
        file_types::try_from_file(path).await
    }

    /// Attempt to determine the `FileType` from a reader synchronously.
    ///
    /// # Errors
    /// if the file type is unknown
    pub fn try_from_reader_sync<R: Read + Seek>(reader: R) -> Result<&'static Self> {
        file_types::try_from_reader_sync(reader, None)
    }

    /// Attempt to determine the `FileType` from a file synchronously.
    ///
    /// # Errors
    /// if the file type is unknown
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
        assert_eq!(file_type.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_type.extensions(), vec!["png".to_string()]);
        assert_eq!(file_type.file_format().id(), 664);
    }

    #[test]
    fn test_from_id_not_found() {
        let file_type = FileType::from_id("fmt/0");
        assert!(file_type.is_none());
    }

    #[test]
    fn test_from_extension() {
        let file_types = FileType::from_extension("md");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "fmt/1149");
        assert_eq!(file_type.media_types(), vec!["text/markdown".to_string()]);
        assert_eq!(file_type.name(), "Markdown");
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
        assert_eq!(
            file_type.extensions(),
            vec!["md".to_string(), "markdown".to_string()]
        );
        assert_eq!(file_type.name(), "Markdown");
    }

    #[test]
    fn test_from_media_type_not_found() {
        let file_types = FileType::from_media_type("foo/bar");
        assert_eq!(0, file_types.len());
    }

    #[test]
    fn test_from_bytes() {
        let value = Vec::new();
        let file_type = FileType::from_bytes(value.as_slice());
        assert_eq!(file_type.id(), "default/1");
        assert_eq!(file_type.name(), "Text");
        assert_eq!(file_type.media_types(), vec!["text/plain".to_string()]);
        assert_eq!(file_type.extensions(), Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_try_from_reader() -> Result<()> {
        let file_path = test_file_path();
        let file = tokio::fs::File::open(file_path).await?;
        let reader = tokio::io::BufReader::new(file);
        let file_type = FileType::try_from_reader(reader).await?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_type.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file(file_path).await?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_type.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> Result<()> {
        let file_path = test_file_path();
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let file_type = FileType::try_from_reader_sync(reader)?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_type.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = FileType::try_from_file_sync(file_path)?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_type.extensions(), vec!["png".to_string()]);
        Ok(())
    }
}
