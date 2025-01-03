use crate::pronom::FileFormat;
use crate::{file_formats, FileType, Result};
use include_dir::{include_dir, Dir, DirEntry};
use quick_xml::de::from_str;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::LazyLock;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek};
use tokio::runtime::Builder;

static FILE_FORMATS: LazyLock<HashMap<String, FileFormat>> = LazyLock::new(file_formats);
static EXTENSION_MAP: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(extension_map);
static MEDIA_TYPE_MAP: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(media_type_map);
static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/");

/// Deserialize the PRONOM XML file format data into a map.
fn file_formats() -> HashMap<String, FileFormat> {
    let mut file_formats = HashMap::new();

    for directory in DATA_DIR.dirs() {
        let formats = directory
            .entries()
            .par_iter()
            .filter_map(DirEntry::as_file)
            .map(|file| {
                let xml = file.contents_utf8().unwrap_or_default();
                from_str(xml).unwrap_or_default()
            })
            .collect::<Vec<FileFormat>>();

        for format in formats {
            let id = format.puid();
            file_formats.insert(id.to_string(), format);
        }
    }

    file_formats
}

/// Create a map of file extensions to file format ids.
fn extension_map() -> HashMap<String, Vec<String>> {
    let mut extension_map = HashMap::new();
    let file_formats = &*FILE_FORMATS;

    for file_format in file_formats.values() {
        for extension in file_format.extensions() {
            let id = file_format.puid();
            let extension = extension.to_string();
            let mut ids = extension_map.get(&extension).unwrap_or(&vec![]).clone();
            ids.push(id.to_string());
            extension_map.insert(extension, ids);
        }
    }

    extension_map
}

/// Create a map of media types to file format ids.
fn media_type_map() -> HashMap<String, Vec<String>> {
    let mut media_type_map = HashMap::new();
    let file_formats = &*FILE_FORMATS;
    for file_format in file_formats.values() {
        for media_type in file_format.media_types() {
            let id = file_format.puid();
            let media_type = media_type.to_string();
            let mut ids = media_type_map.get(&media_type).unwrap_or(&vec![]).clone();
            ids.push(id.to_string());
            media_type_map.insert(media_type, ids);
        }
    }
    media_type_map
}

/// Get the file formats for an id.
pub fn from_id<S: AsRef<str>>(id: S) -> Option<FileFormat> {
    let id = id.as_ref();
    let file_formats = &*FILE_FORMATS;
    file_formats.get(id).cloned()
}

/// Get the file formats for a given extension.
pub fn from_extension<S: AsRef<str>>(extension: S) -> Vec<FileFormat> {
    let extension = extension.as_ref();
    let extension_map = &*EXTENSION_MAP;
    let Some(ids) = extension_map.get(extension) else {
        return vec![];
    };
    let file_formats = &*FILE_FORMATS;
    ids.iter()
        .filter_map(|id| file_formats.get(id))
        .cloned()
        .collect()
}

/// Get the file formats for a given media type.
pub fn from_media_type<S: AsRef<str>>(media_type: S) -> Vec<FileFormat> {
    let media_type = media_type.as_ref();
    let media_type_map = &*MEDIA_TYPE_MAP;
    let Some(ids) = media_type_map.get(media_type) else {
        return vec![];
    };
    let file_formats = &*FILE_FORMATS;
    ids.iter()
        .filter_map(|id| file_formats.get(id))
        .cloned()
        .collect()
}

/// Attempt to determine the `FileFormat` from a byte slice.
pub fn from_bytes<B>(bytes: B, extension: Option<&str>) -> FileFormat
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    let mut file_formats = FILE_FORMATS
        .par_iter()
        .filter(|(_id, file_format)| file_format.is_match(bytes))
        .map(|(id, file_format)| (id.clone(), file_format.clone()))
        .collect::<HashMap<String, FileFormat>>();

    // If there is more than one file format, remove the default formats
    if file_formats.len() > 1 {
        file_formats.remove("default/1");
        file_formats.remove("default/2");
    }

    if file_formats.len() > 1 {
        if let Some(extension) = extension {
            let extension_file_formats = file_formats
                .par_iter()
                .filter(|(_id, file_format)| file_format.extensions().contains(&extension))
                .map(|(id, file_format)| (id.clone(), file_format.clone()))
                .collect::<HashMap<String, FileFormat>>();
            if !extension_file_formats.is_empty() {
                file_formats = extension_file_formats;
            }
        }
    }

    let file_format = if file_formats.is_empty() {
        FILE_FORMATS.get("default/1")
    } else {
        file_formats.values().next()
    };

    file_format.cloned().unwrap_or_default()
}

/// Attempt to determine the `FileFormat` from a reader.
///
/// # Errors
/// if the there is an issue processing the reader
pub async fn try_from_reader<R>(mut reader: R, extension: Option<&str>) -> Result<FileFormat>
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;
    let bytes = buffer.as_slice();
    let file_format = from_bytes(bytes, extension);
    Ok(file_format)
}

/// Attempt to determine the `FileFormat` from a file.
///
/// # Errors
/// if the there is an issue reading the file
pub async fn try_from_file<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    #[cfg(target_arch = "wasm32")]
    let file_format = try_from_file_sync(path);
    #[cfg(not(target_arch = "wasm32"))]
    let file_format = {
        let path = path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str());
        let file = tokio::fs::File::open(path).await?;
        let reader = tokio::io::BufReader::new(file);
        try_from_reader(reader, extension).await
    };
    file_format
}

/// Attempt to determine the `FileFormat` from a reader synchronously.
///
/// # Errors
/// if the file type is unknown
pub fn try_from_reader_sync<R>(mut reader: R, extension: Option<&str>) -> Result<FileFormat>
where
    R: Read + Seek,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let bytes = buffer.as_slice();
    let file_format = from_bytes(bytes, extension);
    Ok(file_format)
}

/// Attempt to determine the `FileFormat` from a file synchronously.
///
/// # Errors
/// if the file type is unknown
pub fn try_from_file_sync<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    let path = path.as_ref();
    let extension = path.extension().and_then(|ext| ext.to_str());
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    try_from_reader_sync(reader, extension)
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
    fn test_file_formats() {
        let file_formats = &*FILE_FORMATS;
        assert_eq!(2608, file_formats.len());
    }

    #[test]
    fn test_extensions() {
        let extensions = &*EXTENSION_MAP;
        assert_eq!(1690, extensions.len());
    }

    #[test]
    fn test_media_types() {
        let media_types = &*MEDIA_TYPE_MAP;
        assert_eq!(301, media_types.len());
        assert!(media_types.contains_key("text/plain"));
        assert!(media_types.contains_key("application/octet-stream"));
    }

    #[test]
    fn test_from_id() {
        let file_format = from_id("fmt/11");
        assert!(file_format.is_some());
        let file_format = file_format.expect("file format");
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_format.extensions(), vec!["png".to_string()]);
    }

    #[test]
    fn test_from_id_not_found() {
        let file_format = from_id("fmt/0");
        assert!(file_format.is_none());
    }

    #[test]
    fn test_from_extension() {
        let file_formats = from_extension("md");
        assert_eq!(1, file_formats.len());
        let file_format = file_formats.first().expect("file format");
        assert_eq!(file_format.puid(), "fmt/1149");
        assert_eq!(file_format.name(), "Markdown");
        assert_eq!(file_format.media_types(), vec!["text/markdown".to_string()]);
        assert_eq!(
            file_format.extensions(),
            vec!["md".to_string(), "markdown".to_string()]
        );
    }

    #[test]
    fn test_from_extension_not_found() {
        let file_formats = from_extension("foo");
        assert!(file_formats.is_empty());
    }

    #[test]
    fn test_from_media_type() {
        let file_formats = from_media_type("text/markdown");
        assert_eq!(1, file_formats.len());
        let file_format = file_formats.first().expect("file format");
        assert_eq!(file_format.puid(), "fmt/1149");
        assert_eq!(file_format.name(), "Markdown");
        assert_eq!(file_format.media_types(), vec!["text/markdown".to_string()]);
        assert_eq!(
            file_format.extensions(),
            vec!["md".to_string(), "markdown".to_string()]
        );
    }

    #[test]
    fn test_from_media_type_not_found() {
        let file_formats = from_media_type("foo/bar");
        assert!(file_formats.is_empty());
    }

    #[test]
    fn test_from_bytes() {
        let value = b"\xCA\xFE\xBA\xBE".to_vec();
        let file_format = from_bytes(value.as_slice(), None);
        assert_eq!(file_format.puid(), "x-fmt/415");
        assert_eq!(file_format.name(), "Java Class File");
        assert_eq!(file_format.media_types(), Vec::<String>::new());
        assert_eq!(file_format.extensions(), vec!["class".to_string()]);
    }

    #[tokio::test]
    async fn test_try_from_reader() -> Result<()> {
        let file_path = test_file_path();
        let file = tokio::fs::File::open(file_path).await?;
        let reader = tokio::io::BufReader::new(file);
        let file_format = try_from_reader(reader, None).await?;
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_format.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_format = try_from_file(file_path).await?;
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_format.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> Result<()> {
        let file_path = test_file_path();
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let file_format = try_from_reader_sync(reader, None)?;
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_format.extensions(), vec!["png".to_string()]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_format = try_from_file_sync(file_path)?;
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.media_types(), vec!["image/png".to_string()]);
        assert_eq!(file_format.extensions(), vec!["png".to_string()]);
        Ok(())
    }
}
