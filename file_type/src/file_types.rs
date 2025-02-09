use crate::format::{Regex, RelationshipType, UNIDENTIFIED_KEY};
use crate::{signatures, sources, Error, FileType, Result};
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cmp::Ordering;
use sources::default::{DEFAULT_1, DEFAULT_2};
use std::io::Read;
use std::path::Path;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

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
    if let Some(signatures) = signatures::MAP.get(&signature_key) {
        file_types.extend(
            signatures
                .iter()
                .filter(|file_type| file_type.file_format().is_match(bytes)),
        );
    }
    // Get all file types with unidentified signature key; these are the file types that did not
    // have a BOF literal signature.
    if let Some(signatures) = signatures::MAP.get(&UNIDENTIFIED_KEY) {
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
                if let Some(file_type) = FileType::from_extension(extension).first() {
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
    } else if is_binary(bytes) {
        &DEFAULT_1
    } else {
        &DEFAULT_2
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
    use crate::format::SourceType;
    use crate::sources::file_types;
    use std::path::PathBuf;

    const TEST_FILE_NAME: &str = "pronom-664-signature-0.png";

    fn test_file_path() -> PathBuf {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(crate_dir)
            .join("..")
            .join("test_data")
            .join("pronom")
            .join(TEST_FILE_NAME)
    }

    fn find_file_type(source_type: &SourceType, id: usize) -> &'static FileType {
        let file_type = file_types().find(|file_type| {
            file_type.file_format().source_type == *source_type && file_type.file_format().id == id
        });
        file_type.expect("file type not found")
    }

    #[test]
    fn test_file_formats() {
        let default_1 = find_file_type(&SourceType::Default, 1);
        assert_eq!(default_1.id(), 1);
        let default_2 = find_file_type(&SourceType::Default, 2);
        assert_eq!(default_2.id(), 2);
    }

    #[test]
    fn test_from_bytes() {
        let value = b"\xCA\xFE\xBA\xBE".to_vec();
        let file_type = from_bytes(value.as_slice(), None);
        assert_eq!(file_type.extensions(), vec!["class"]);
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_reader() -> anyhow::Result<()> {
        let file_path = test_file_path();
        let file = tokio::fs::File::open(file_path).await?;
        let reader = tokio::io::BufReader::new(file);
        let file_type = try_from_reader(reader, None).await?;
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "tokio")]
    async fn test_try_from_file() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file(file_path).await?;
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
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[test]
    fn test_try_from_file_sync() -> Result<()> {
        let file_path = test_file_path();
        let file_type = try_from_file_sync(file_path)?;
        assert_eq!(file_type.media_types(), vec!["image/png"]);
        assert_eq!(file_type.extensions(), vec!["png"]);
        Ok(())
    }

    #[cfg(feature = "pronom")]
    #[test]
    fn test_cmp_file_types() {
        let pronom_654 = find_file_type(&SourceType::Pronom, 654);
        let pronom_1314 = find_file_type(&SourceType::Pronom, 1314);
        let pronom_1507 = find_file_type(&SourceType::Pronom, 1507);
        let mut file_types = [pronom_654, pronom_1314, pronom_1507];

        sort_by(&mut file_types, cmp_file_types);

        assert_eq!(file_types[0].id(), 1507);
        assert_eq!(file_types[1].id(), 1314);
        assert_eq!(file_types[2].id(), 654);
    }

    #[cfg(feature = "pronom")]
    #[test]
    fn test_cmp_file_type_extensions() {
        let pronom_940 = find_file_type(&SourceType::Pronom, 940);
        let pronom_1281 = find_file_type(&SourceType::Pronom, 1281);
        let pronom_2679 = find_file_type(&SourceType::Pronom, 2679);
        let mut file_types = [pronom_2679, pronom_940, pronom_1281];

        sort_by(&mut file_types, cmp_file_type_extensions);

        assert_eq!(file_types[0].id(), 940);
        assert_eq!(file_types[1].id(), 1281);
        assert_eq!(file_types[2].id(), 2679);
    }
}
