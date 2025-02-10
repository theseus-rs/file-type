use crate::format::{FileFormat, SourceType};
use crate::{extensions, file_types, media_types};
use core::cmp::Ordering;
use std::io::Read;

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
/// Retrieve a file type from an extension:
/// ```
/// use file_type::FileType;
///
/// let file_types = FileType::from_extension("png");
/// let file_type = file_types.first().expect("file format");
/// assert_eq!(file_type.media_types(), vec!["image/png"]);
/// ```
///
/// Retrieve a file type from a media type:
/// ```
/// use file_type::FileType;
///
/// let file_types = FileType::from_media_type("image/png");
/// let file_type = file_types.first().expect("file format");
/// assert_eq!(file_type.extensions(), vec!["png"]);
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
        let extension = extension.as_ref();
        let Some(extensions) = extensions::MAP.get(extension) else {
            return &[];
        };
        extensions
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
        let media_type = media_type.as_ref();
        let Some(media_types) = media_types::MAP.get(media_type) else {
            return &[];
        };
        media_types
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
    /// use std::io::BufReader;
    ///
    /// let bytes = b"\xCA\xFE\xBA\xBE";
    /// let reader = BufReader::new(&bytes[..]);
    /// let file_type = FileType::try_from_reader(reader).expect("file type not found");
    /// assert_eq!(file_type.extensions(), vec!["class"]);
    /// assert_eq!(file_type.media_types(), Vec::<String>::new());
    /// ```
    #[cfg(feature = "std")]
    pub fn try_from_reader<R: std::io::Read>(mut reader: R) -> crate::Result<&'static Self> {
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|error| crate::Error::new(error.to_string()))?;
        let bytes = buffer.as_slice();
        let file_type = file_types::from_bytes(bytes, None);
        Ok(file_type)
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
    /// let file_path = Path::new("image.png");
    /// let file_type = FileType::try_from_file(file_path).expect("file type not found");
    /// assert_eq!(file_type.extensions(), vec!["png"]);
    /// assert_eq!(file_type.media_types(), vec!["image/png"]);
    /// ```
    #[cfg(feature = "std")]
    pub fn try_from_file<P: AsRef<std::path::Path>>(path: P) -> crate::Result<&'static Self> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str());
        let file =
            std::fs::File::open(path).map_err(|error| crate::Error::new(error.to_string()))?;
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|error| crate::Error::new(error.to_string()))?;
        let bytes = buffer.as_slice();
        let file_type = file_types::from_bytes(bytes, extension);
        Ok(file_type)
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
    fn test_media_types() {
        assert!(!media_types::MAP.is_empty());
        assert!(media_types::MAP.contains_key("text/plain"));
        assert!(media_types::MAP.contains_key("application/octet-stream"));
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

    #[cfg(feature = "std")]
    #[test]
    fn test_try_from_reader() -> crate::Result<()> {
        let bytes = b"\xCA\xFE\xBA\xBE";
        let reader = std::io::BufReader::new(&bytes[..]);
        let file_type = FileType::try_from_reader(reader)?;
        assert_eq!(file_type.extensions(), vec!["class"]);
        Ok(())
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_try_from_file() -> crate::Result<()> {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = std::path::PathBuf::from(crate_dir)
            .join("..")
            .join("test_data")
            .join("pronom")
            .join("pronom-664-signature-0.png");
        let file_type = FileType::try_from_file(file_path)?;
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
