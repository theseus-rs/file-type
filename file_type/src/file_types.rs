use crate::format::{FileFormat, InternalSignature, Regex, RelationshipType};
use crate::sources::FILE_FORMATS;
use crate::{file_types, sources, Error, FileType, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::LazyLock;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

static FILE_TYPES: LazyLock<HashMap<&'static str, FileType>> =
    LazyLock::new(initialize_file_formats);
static SIGNATURE_MAP: LazyLock<HashMap<u64, Vec<&'static FileType>>> =
    LazyLock::new(initialize_signature_map);
static EXTENSION_MAP: LazyLock<HashMap<&'static str, Vec<&'static FileType>>> =
    LazyLock::new(initialize_extension_map);
static MEDIA_TYPE_MAP: LazyLock<HashMap<&'static str, Vec<&'static FileType>>> =
    LazyLock::new(initialize_media_type_map);
const EMPTY_SIGNATURES: &Vec<&'static FileType> = &Vec::new();
const EMPTY_EXTENSIONS: &Vec<&'static FileType> = &Vec::new();
const EMPTY_MEDIA_TYPES: &Vec<&'static FileType> = &Vec::new();

/// Deserialize the PRONOM XML file format data into a map of puid to `FileType`.
fn initialize_file_formats() -> HashMap<&'static str, FileType> {
    let mut file_types = HashMap::new();
    for file_formats in FILE_FORMATS {
        for file_format in *file_formats {
            let file_format = *file_format;
            let file_type = FileType::new(file_format);
            let id = file_type.id();
            file_types.insert(id, file_type);
        }
    }
    file_types
}

/// Create a list of file types with signatures
fn initialize_signature_map() -> HashMap<u64, Vec<&'static FileType>> {
    let mut signatures = HashMap::new();

    for file_type in FILE_TYPES.values() {
        let file_format = file_type.file_format();
        let internal_signature_keys = file_format
            .internal_signatures
            .iter()
            .map(InternalSignature::key)
            .collect::<Vec<u64>>();
        for key in internal_signature_keys {
            let mut file_types: Vec<&FileType> = signatures.remove(&key).unwrap_or_default();
            file_types.push(file_type);
            signatures.insert(key, file_types);
        }
    }

    signatures
}

/// Create a map of file extensions to file types.
fn initialize_extension_map() -> HashMap<&'static str, Vec<&'static FileType>> {
    let mut extension_map = HashMap::new();

    for file_type in FILE_TYPES.values() {
        for extension in file_type.extensions() {
            let extension = *extension;
            let mut file_types = extension_map.get(extension).unwrap_or(&vec![]).clone();
            file_types.push(file_type);
            file_types.sort();
            extension_map.insert(extension, file_types);
        }
    }

    extension_map
}

/// Create a map of media types to file types.
fn initialize_media_type_map() -> HashMap<&'static str, Vec<&'static FileType>> {
    let mut media_type_map = HashMap::new();
    for file_type in FILE_TYPES.values() {
        for media_type in file_type.media_types() {
            let media_type = *media_type;
            let mut file_types = media_type_map.get(media_type).unwrap_or(&vec![]).clone();
            file_types.push(file_type);
            file_types.sort();
            media_type_map.insert(media_type, file_types);
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

/// Sort the file types without requiring a total order.
fn sort_by<F, T>(file_types: &mut [&T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    for i in 1..file_types.len() {
        let mut j = i;
        while j > 0 && compare(file_types[j - 1], file_types[j]) == Ordering::Greater {
            file_types.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Compare the file types based on their priority; this is used to determine the most appropriate
/// file type for a given file.  The order of the file types is determined by the relationship
/// between the file formats.
fn cmp_file_types(a: &FileType, b: &FileType) -> Ordering {
    let a_file_format = a.file_format();
    let b_file_format = b.file_format();
    let a_id = a_file_format.id;
    let b_id = b_file_format.id;

    for related_format in a_file_format.related_formats {
        if b_id != related_format.id {
            continue;
        }

        match related_format.relationship_type {
            RelationshipType::HasLowerPriorityThan => {
                return Ordering::Greater;
            }
            RelationshipType::HasPriorityOver => {
                return Ordering::Less;
            }
            _ => {}
        }
    }

    for related_format in b_file_format.related_formats {
        if a_id != related_format.id {
            continue;
        }

        match related_format.relationship_type {
            RelationshipType::HasLowerPriorityThan => {
                return Ordering::Less;
            }
            RelationshipType::HasPriorityOver => {
                return Ordering::Greater;
            }
            _ => {}
        }
    }

    a_file_format.cmp(b_file_format)
}

/// Compare the file types based on their priority and id.  The most general file type should be
/// sorted first since these file types were not identified by signature. The order of the file
/// types is determined by the relationship between the file formats.
fn cmp_file_type_extensions(a: &FileType, b: &FileType) -> Ordering {
    let a_file_format = a.file_format();
    let b_file_format = b.file_format();
    let a_id = a_file_format.id;
    let b_id = b_file_format.id;

    for related_format in a_file_format.related_formats {
        if b_id != related_format.id {
            continue;
        }

        match related_format.relationship_type {
            RelationshipType::HasLowerPriorityThan => {
                return Ordering::Less;
            }
            RelationshipType::HasPriorityOver => {
                return Ordering::Greater;
            }
            _ => {}
        }
    }

    for related_format in b_file_format.related_formats {
        if a_id != related_format.id {
            continue;
        }

        match related_format.relationship_type {
            RelationshipType::HasLowerPriorityThan => {
                return Ordering::Greater;
            }
            RelationshipType::HasPriorityOver => {
                return Ordering::Less;
            }
            _ => {}
        }
    }

    a_file_format.cmp(b_file_format)
}

/// Determines if a byte slice is binary or text data.
fn is_binary(bytes: &[u8]) -> bool {
    bytes.is_empty()
        || bytes
            .iter()
            .any(|&byte| matches!(byte, 0..=31 if !matches!(byte, b'\n' | b'\r' | b'\t')))
}

/// Attempt to determine the `FileType` from a byte slice.
pub(crate) fn from_bytes<B>(bytes: B, extension: Option<&str>) -> &'static FileType
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    let signature_key = Regex::key_from_bytes(bytes);
    let mut file_types: Vec<&'static FileType> = Vec::new();
    if let Some(signatures) = SIGNATURE_MAP.get(&signature_key) {
        file_types.extend(
            signatures
                .iter()
                .filter(|file_type| file_type.file_format().is_match(bytes)),
        );
    }
    // Get all file types with a signature of 0; these are the file types that did not have a
    // BOF literal signature.
    if let Some(signatures) = SIGNATURE_MAP.get(&0) {
        file_types.extend(
            signatures
                .iter()
                .filter(|file_type| file_type.file_format().is_match(bytes)),
        );
    }

    match file_types.len() {
        0 => {
            if let Some(extension) = extension {
                // The extensions are pre-sorted; return the first one found as the best match
                if let Some(file_type) = from_extension(extension).first() {
                    return file_type;
                };
            }
        }
        1 => {}
        _ => {
            if let Some(extension) = extension {
                file_types.retain(|file_type| file_type.extensions().contains(&extension));
            }
            sort_by(&mut file_types, cmp_file_types);
        }
    }

    if let Some(file_type) = file_types.first() {
        file_type
    } else {
        let default_id = if is_binary(bytes) {
            "default/1"
        } else {
            "default/2"
        };
        FILE_TYPES.get(default_id).expect("No file type found")
    }
}

/// Attempt to determine the `FileType` from a reader.
///
/// # Errors
/// if the there is an issue processing the reader
#[cfg(feature = "tokio")]
pub(crate) async fn try_from_reader<R>(
    mut reader: R,
    extension: Option<&str>,
) -> Result<&'static FileType>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .await
        .map_err(|error| Error::new(error.to_string()))?;
    let bytes = buffer.as_slice();
    let file_type = from_bytes(bytes, extension);
    Ok(file_type)
}

/// Attempt to determine the `FileType` from a file.
///
/// # Errors
/// if the there is an issue reading the file
#[cfg(feature = "tokio")]
pub(crate) async fn try_from_file<P: AsRef<Path>>(path: P) -> Result<&'static FileType> {
    #[cfg(target_arch = "wasm32")]
    let file_type = try_from_file_sync(path);
    #[cfg(not(target_arch = "wasm32"))]
    let file_type = {
        let path = path.as_ref();
        let extension = path.extension().and_then(|extension| extension.to_str());
        let file = tokio::fs::File::open(path)
            .await
            .map_err(|error| Error::new(error.to_string()))?;
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
    reader
        .read_to_end(&mut buffer)
        .map_err(|error| Error::new(error.to_string()))?;
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
    let file = std::fs::File::open(path).map_err(|error| Error::new(error.to_string()))?;
    let reader = std::io::BufReader::new(file);
    try_from_reader_sync(reader, extension)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
    const TEST_FILE_NAME: &str = "pronom-664-signature-id-58.png";

    fn test_file_path() -> PathBuf {
        let path = format!("{CRATE_DIR}/../testdata/pronom/{TEST_FILE_NAME}");
        PathBuf::from(path)
    }

    #[test]
    fn test_file_formats() {
        let file_types = &*FILE_TYPES;
        assert!(!file_types.is_empty());
        assert!(file_types.contains_key("default/1"));
        assert!(file_types.contains_key("default/2"));
    }

    #[test]
    fn test_extensions() {
        let extensions = &*EXTENSION_MAP;
        assert!(!extensions.is_empty());
    }

    #[test]
    fn test_media_types() {
        let media_types = &*MEDIA_TYPE_MAP;
        assert!(!media_types.is_empty());
        assert!(media_types.contains_key("text/plain"));
        assert!(media_types.contains_key("application/octet-stream"));
    }

    #[test]
    fn test_from_id() {
        let file_type = from_id("pronom/664").expect("file format");
        assert_eq!(file_type.id(), "pronom/664");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
    }

    #[test]
    fn test_from_id_not_found() {
        let file_type = from_id("pronom/0");
        assert!(file_type.is_none());
    }

    #[test]
    fn test_from_extension() {
        let file_types = from_extension("duckdb");
        assert_eq!(1, file_types.len());
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "custom/3");
        assert_eq!(file_type.name(), "DuckDB");
        assert_eq!(file_type.media_types(), vec!["application/vnd.duckdb.file"]);
        assert_eq!(file_type.extensions(), vec!["duckdb"]);
    }

    #[test]
    fn test_from_extension_not_found() {
        let file_types = from_extension("foo");
        assert!(file_types.is_empty());
    }

    #[test]
    fn test_from_media_type() {
        let file_types = from_media_type("text/markdown");
        let file_type = file_types.first().expect("file format");
        assert_eq!(file_type.id(), "pronom/1959");
        assert_eq!(file_type.name(), "Markdown");
        assert_eq!(file_type.media_types(), vec!["text/markdown"]);
        assert_eq!(file_type.extensions(), vec!["md", "markdown"]);
    }

    #[test]
    fn test_from_bytes() {
        let value = b"\xCA\xFE\xBA\xBE".to_vec();
        let file_type = from_bytes(value.as_slice(), None);
        assert_eq!(file_type.id(), "pronom/802");
        assert_eq!(file_type.name(), "Java Class File");
        assert_eq!(file_type.media_types(), Vec::<String>::new());
        assert_eq!(file_type.extensions(), vec!["class"]);
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_reader() -> anyhow::Result<()> {
        let file_path = test_file_path();
        let file = tokio::fs::File::open(file_path).await?;
        let reader = tokio::io::BufReader::new(file);
        let file_type = try_from_reader(reader, None).await?;
        assert_eq!(file_type.id(), "pronom/664");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file(file_path).await?;
        assert_eq!(file_type.id(), "pronom/664");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_reader_sync() -> anyhow::Result<()> {
        let file_path = test_file_path();
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let file_type = try_from_reader_sync(reader, None)?;
        assert_eq!(file_type.id(), "pronom/664");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file_sync(file_path)?;
        assert_eq!(file_type.id(), "pronom/664");
        assert_eq!(file_type.name(), "Portable Network Graphics");
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_cmp_file_types() {
        let pronom_654 = FileType::from_id("pronom/654").expect("file type not found");
        let pronom_1314 = FileType::from_id("pronom/1314").expect("file type not found");
        let pronom_1507 = FileType::from_id("pronom/1507").expect("file type not found");
        let mut file_types = [pronom_654, pronom_1314, pronom_1507];

        sort_by(&mut file_types, cmp_file_types);

        assert_eq!(file_types[0].id(), "pronom/1507");
        assert_eq!(file_types[1].id(), "pronom/1314");
        assert_eq!(file_types[2].id(), "pronom/654");
    }

    #[test]
    fn test_cmp_file_type_extensions() {
        let pronom_940 = FileType::from_id("pronom/940").expect("file type not found");
        let pronom_1281 = FileType::from_id("pronom/1281").expect("file type not found");
        let pronom_2679 = FileType::from_id("pronom/2679").expect("file type not found");
        let mut file_types = [pronom_2679, pronom_940, pronom_1281];

        sort_by(&mut file_types, cmp_file_type_extensions);

        assert_eq!(file_types[0].id(), "pronom/940");
        assert_eq!(file_types[1].id(), "pronom/1281");
        assert_eq!(file_types[2].id(), "pronom/2679");
    }

    #[test]
    fn create_supported_formats() -> anyhow::Result<()> {
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
