use crate::format::{FileFormat, RelationshipType};
use crate::{file_types, FileType, Result};
use include_dir::{include_dir, Dir, DirEntry};
use quick_xml::de::from_str;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::LazyLock;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek};
use tokio::runtime::Builder;

static FILE_TYPES: LazyLock<HashMap<String, FileType>> = LazyLock::new(initialize_file_formats);
static SIGNATURE_MAP: LazyLock<HashMap<&'static str, &'static FileType>> =
    LazyLock::new(initialize_signature_map);
static EXTENSION_MAP: LazyLock<HashMap<&'static str, Vec<&'static FileType>>> =
    LazyLock::new(initialize_extension_map);
static MEDIA_TYPE_MAP: LazyLock<HashMap<&'static str, Vec<&'static FileType>>> =
    LazyLock::new(initialize_media_type_map);
static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/");
const EMPTY_EXTENSIONS: &Vec<&'static FileType> = &Vec::new();
const EMPTY_MEDIA_TYPES: &Vec<&'static FileType> = &Vec::new();
// The following file types are slow to process and should be skipped when determining the file type
const SLOW_FILE_TYPES: [&str; 20] = [
    "fmt/63", "fmt/64", "fmt/65", "fmt/66", "fmt/67", "fmt/68", "fmt/69", "fmt/70", "fmt/71",
    "fmt/72", "fmt/73", "fmt/74", "fmt/75", "fmt/76", "fmt/77", "fmt/78", "fmt/79", "fmt/433",
    "fmt/435", "fmt/1389",
];

/// Deserialize the PRONOM XML file format data into a map of puid to `FileType`.
fn initialize_file_formats() -> HashMap<String, FileType> {
    let mut file_types = HashMap::new();

    for directory in DATA_DIR.dirs() {
        let file_formats = directory
            .entries()
            .par_iter()
            .filter_map(DirEntry::as_file)
            .map(|file| {
                let xml = file.contents_utf8().unwrap_or_default();
                from_str(xml).unwrap_or_default()
            })
            .collect::<Vec<FileFormat>>();

        for file_format in file_formats {
            let id = file_format.puid().to_string();
            let file_type = FileType::new(file_format);
            file_types.insert(id, file_type);
        }
    }

    file_types
}

/// Create a listfile types with signatures
fn initialize_signature_map() -> HashMap<&'static str, &'static FileType> {
    let mut signatures = HashMap::new();
    let file_types = &*FILE_TYPES;

    for file_type in file_types.values() {
        let id = file_type.id();
        if SLOW_FILE_TYPES.contains(&id) {
            continue;
        }

        let file_format = file_type.file_format();
        let internal_signatures = file_format.internal_signatures();
        if !internal_signatures.is_empty() {
            signatures.insert(id, file_type);
        }
    }

    signatures
}

/// Create a map of file extensions to file types.
fn initialize_extension_map() -> HashMap<&'static str, Vec<&'static FileType>> {
    let mut extension_map = HashMap::new();
    let file_types = &*FILE_TYPES;

    for file_type in file_types.values() {
        for extension in file_type.extensions() {
            let mut types = extension_map.get(&extension).unwrap_or(&vec![]).clone();
            types.push(file_type);
            extension_map.insert(extension, types);
        }
    }

    extension_map
}

/// Create a map of media types to file types.
fn initialize_media_type_map() -> HashMap<&'static str, Vec<&'static FileType>> {
    let mut media_type_map = HashMap::new();
    let file_types = &*FILE_TYPES;
    for file_type in file_types.values() {
        for media_type in file_type.media_types() {
            let mut types = media_type_map.get(&media_type).unwrap_or(&vec![]).clone();
            types.push(file_type);
            media_type_map.insert(media_type, types);
        }
    }
    media_type_map
}

/// Get the file type for an id.
pub(crate) fn from_id<S: AsRef<str>>(id: S) -> Option<&'static FileType> {
    let id = id.as_ref();
    let file_formats = &*FILE_TYPES;
    file_formats.get(id)
}

/// Get the file types for a given extension.
pub(crate) fn from_extension<S: AsRef<str>>(extension: S) -> &'static Vec<&'static FileType> {
    let extension = extension.as_ref();
    EXTENSION_MAP.get(extension).unwrap_or(EMPTY_EXTENSIONS)
}

/// Get the file types for a given media type.
pub(crate) fn from_media_type<S: AsRef<str>>(media_type: S) -> &'static Vec<&'static FileType> {
    let media_type = media_type.as_ref();
    MEDIA_TYPE_MAP.get(media_type).unwrap_or(EMPTY_MEDIA_TYPES)
}

/// Sort the file types based on their priority; this is used to determine the most appropriate file
/// type for a given file.  The order of the file types is determined by the relationship between
/// the file formats.  The standard sort method cannot be used because the comparison function
/// does not support a total order.
pub(crate) fn sort_file_types(file_types: &mut Vec<&FileType>) {
    let len = file_types.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if cmp_file_types(file_types[j], file_types[j + 1]) == Ordering::Greater {
                file_types.swap(j, j + 1);
            }
        }
    }
}

fn cmp_file_types(a: &FileType, b: &FileType) -> Ordering {
    let other_id = b.file_format().id();
    for related_format in a.file_format().related_formats() {
        if other_id != related_format.id() {
            continue;
        }

        match related_format.relationship_type() {
            RelationshipType::HasLowerPriorityThan => {
                return Ordering::Greater;
            }
            RelationshipType::HasPriorityOver => {
                return Ordering::Less;
            }
            _ => {}
        }
    }

    let self_id = a.file_format().id();
    self_id.cmp(&other_id)
}

/// Determines if a byte slice is binary or text data.
fn is_binary(bytes: &[u8]) -> bool {
    bytes.is_empty()
        || bytes
            .iter()
            .any(|byte| *byte < 32 && ![b'\n', b'\r', b'\t'].contains(byte))
}

/// Attempt to determine the `FileType` from a byte slice.
pub(crate) fn from_bytes<B>(bytes: B, extension: Option<&str>) -> &'static FileType
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    let file_types: HashMap<&str, &'static FileType> = SIGNATURE_MAP
        .par_iter()
        .filter(|(_id, file_type)| file_type.file_format().is_match(bytes))
        .map(|(id, file_type)| (*id, *file_type))
        .collect();

    let mut file_types = file_types.into_values().collect::<Vec<&'static FileType>>();

    match file_types.len() {
        0 => {
            if let Some(extension) = extension {
                let extension_map = &*EXTENSION_MAP;
                if let Some(types) = extension_map.get(extension) {
                    file_types = Vec::new();
                    for file_type in types {
                        file_types.push(file_type);
                    }
                    // If no file formats were found, sort the file types in reverse order in order
                    // to prioritize the most generic file type
                    sort_file_types(&mut file_types);
                    file_types.reverse();
                };
            }
        }
        1 => {}
        _ => {
            if let Some(extension) = extension {
                let mut types = Vec::new();
                for file_type in &file_types {
                    if file_type.extensions().contains(&extension) {
                        types.push(*file_type);
                    }
                }
                if !types.is_empty() {
                    file_types = types;
                }
            }
            sort_file_types(&mut file_types);
        }
    }

    let file_type = if file_types.is_empty() {
        if is_binary(bytes) {
            FILE_TYPES.get("default/1")
        } else {
            FILE_TYPES.get("default/2")
        }
    } else {
        file_types.into_iter().next()
    };

    let Some(file_type) = file_type else {
        unreachable!("No file type found");
    };
    file_type
}

/// Attempt to determine the `FileType` from a reader.
///
/// # Errors
/// if the there is an issue processing the reader
pub(crate) async fn try_from_reader<R>(
    mut reader: R,
    extension: Option<&str>,
) -> Result<&'static FileType>
where
    R: AsyncRead + Unpin,
{
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;
    let bytes = buffer.as_slice();
    let file_type = from_bytes(bytes, extension);
    Ok(file_type)
}

/// Attempt to determine the `FileType` from a file.
///
/// # Errors
/// if the there is an issue reading the file
pub(crate) async fn try_from_file<P: AsRef<Path>>(path: P) -> Result<&'static FileType> {
    #[cfg(target_arch = "wasm32")]
    let file_type = try_from_file_sync(path);
    #[cfg(not(target_arch = "wasm32"))]
    let file_type = {
        let path = path.as_ref();
        let extension = path.extension().and_then(|extension| extension.to_str());
        let file = tokio::fs::File::open(path).await?;
        let reader = tokio::io::BufReader::new(file);
        try_from_reader(reader, extension).await
    };
    file_type
}

/// Attempt to determine the `FileType` from a reader synchronously.
///
/// # Errors
/// if the file type is unknown
pub(crate) fn try_from_reader_sync<R: Read>(
    mut reader: R,
    extension: Option<&str>,
) -> Result<&'static FileType> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let bytes = buffer.as_slice();
    let file_type = from_bytes(bytes, extension);
    Ok(file_type)
}

/// Attempt to determine the `FileType` from a file synchronously.
///
/// # Errors
/// if the file type is unknown
pub(crate) fn try_from_file_sync<P: AsRef<Path>>(path: P) -> Result<&'static FileType> {
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
        let file_types = &*FILE_TYPES;
        assert_eq!(3844, file_types.len());
    }

    #[test]
    fn test_extensions() {
        let extensions = &*EXTENSION_MAP;
        assert_eq!(3608, extensions.len());
    }

    #[test]
    fn test_media_types() {
        let media_types = &*MEDIA_TYPE_MAP;
        assert_eq!(1025, media_types.len());
        assert!(media_types.contains_key("text/plain"));
        assert!(media_types.contains_key("application/octet-stream"));
    }

    #[test]
    fn test_from_id() {
        let file_type = from_id("fmt/11").expect("file format");
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
    }

    #[test]
    fn test_from_id_not_found() {
        let file_type = from_id("fmt/0");
        assert!(file_type.is_none());
    }

    #[test]
    fn test_from_extension() {
        let file_types = from_extension("md");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "fmt/1149");
        assert_eq!(file_type.name(), "Markdown");
        assert_eq!(file_type.media_types(), vec!["text/markdown"]);
        assert_eq!(file_type.extensions(), vec!["md", "markdown"]);
    }

    #[test]
    fn test_from_extension_not_found() {
        let file_types = from_extension("foo");
        assert!(file_types.is_empty());
    }

    #[test]
    fn test_from_media_type() {
        let file_types = from_media_type("text/markdown");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "fmt/1149");
        assert_eq!(file_type.name(), "Markdown");
        assert_eq!(file_type.media_types(), vec!["text/markdown"]);
        assert_eq!(file_type.extensions(), vec!["md", "markdown"]);
    }

    #[test]
    fn test_from_bytes() {
        let value = b"\xCA\xFE\xBA\xBE".to_vec();
        let file_type = from_bytes(value.as_slice(), None);
        assert_eq!(file_type.id(), "x-fmt/415");
        assert_eq!(file_type.name(), "Java Class File");
        assert_eq!(file_type.media_types(), Vec::<String>::new());
        assert_eq!(file_type.extensions(), vec!["class"]);
    }

    #[tokio::test]
    async fn test_try_from_reader() -> Result<()> {
        let file_path = test_file_path();
        let file = tokio::fs::File::open(file_path).await?;
        let reader = tokio::io::BufReader::new(file);
        let file_type = try_from_reader(reader, None).await?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file(file_path).await?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> Result<()> {
        let file_path = test_file_path();
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let file_type = try_from_reader_sync(reader, None)?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file_sync(file_path)?;
        assert_eq!(file_type.id(), "fmt/11");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn create_supported_formats() -> Result<()> {
        let mut file_types = FILE_TYPES.values().collect::<Vec<_>>();
        file_types.sort_by_key(|a| format!("{}|{}", a.name().to_lowercase(), a.id()));

        let file_types = file_types
            .iter()
            .map(|file_type| {
                let id = file_type.id();
                let name = file_type.name();
                let media_types = file_type.media_types().join(", ");
                let extensions = file_type.extensions().join(", ");
                format!("| {id} | {name} | {extensions} | {media_types} |")
            })
            .collect::<Vec<String>>();

        let file_types = file_types.join("\n");
        let file_types = [
            format!("# File Types ({})\n", FILE_TYPES.len()),
            "| Id | Name | Extensions | Media Types |".to_string(),
            "| ---- | ---- | ----------- | ---------- |".to_string(),
            file_types,
        ]
        .join("\n");

        let file_name = PathBuf::from(CRATE_DIR).join("..").join("FILETYPES.md");
        std::fs::write(file_name, file_types)?;
        Ok(())
    }
}
