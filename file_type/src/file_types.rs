use crate::format::{Regex, RelationshipType, UNIDENTIFIED_KEY};
use crate::{FileType, signatures, sources};
use alloc::vec::Vec;
use core::cmp::Ordering;
use sources::default::{DEFAULT_1, DEFAULT_2};

/// Sort the file types without requiring a total order.
#[expect(
    clippy::indexing_slicing,
    reason = "j starts at a valid index and stays in bounds while it is greater than zero"
)]
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

/// Determines if a byte slice is binary or text data.
fn is_binary(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return true;
    }

    // Check only the first portion of the file for performance
    bytes
        .iter()
        .take(8192)
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
                let extension = extension.to_lowercase();
                // The extensions are pre-sorted; return the first one found as the best match
                if let Some(file_type) = FileType::from_extension(extension).first() {
                    return file_type;
                }
            }
        }
        1 => {}
        _ => {
            if let Some(extension) = extension {
                let extension = extension.to_lowercase();
                file_types.retain(|file_type| file_type.extensions().contains(&&*extension));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
    use crate::sources::file_types;
    use alloc::vec;

    static LOWER_PRIORITY_FORMAT: FileFormat = FileFormat {
        id: 1,
        source_type: SourceType::Default,
        name: "Lower priority",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2,
        }],
    };
    static LOWER_PRIORITY: FileType = FileType {
        file_format: &LOWER_PRIORITY_FORMAT,
    };
    static HIGHER_PRIORITY_FORMAT: FileFormat = FileFormat {
        id: 2,
        source_type: SourceType::Default,
        name: "Higher priority",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    };
    static HIGHER_PRIORITY: FileType = FileType {
        file_format: &HIGHER_PRIORITY_FORMAT,
    };

    fn find_file_type(source_type: &SourceType, id: usize) -> Option<&'static FileType> {
        file_types().find(|file_type| {
            file_type.file_format().source_type == *source_type && file_type.file_format().id == id
        })
    }

    #[test]
    fn test_file_formats() {
        assert_eq!(
            find_file_type(&SourceType::Default, 1).map(FileType::id),
            Some(1)
        );
        assert_eq!(
            find_file_type(&SourceType::Default, 2).map(FileType::id),
            Some(2)
        );
    }

    #[test]
    fn test_from_bytes() {
        let value = b"\xCA\xFE\xBA\xBE".to_vec();
        let file_type = from_bytes(value.as_slice(), None);
        assert_eq!(file_type.extensions(), vec!["class"]);
    }

    #[test]
    fn test_lookup_maps_are_complete_and_sorted() {
        for file_type in file_types() {
            let file_format = file_type.file_format();

            for extension in file_format.extensions {
                assert!(
                    crate::extensions::MAP
                        .get(extension)
                        .is_some_and(|matches| matches.contains(&file_type)),
                    "missing extension {extension:?} for {file_format:?}",
                );
            }

            for media_type in file_format.media_types {
                assert!(
                    crate::media_types::MAP
                        .get(media_type)
                        .is_some_and(|matches| matches.contains(&file_type)),
                    "missing media type {media_type:?} for {file_format:?}",
                );
            }

            for signature in file_format.signatures {
                let key = signature.key();
                assert!(
                    signatures::MAP
                        .get(&key)
                        .is_some_and(|matches| matches.contains(&file_type)),
                    "missing signature key {key} for {file_format:?}",
                );
            }
        }

        for matches in crate::extensions::MAP.values() {
            assert!(matches.is_sorted());
        }
        for matches in crate::media_types::MAP.values() {
            assert!(matches.is_sorted());
        }
        for matches in signatures::MAP.values() {
            assert!(matches.is_sorted());
        }
    }

    #[test]
    fn test_cmp_file_types() {
        let mut file_types = [&LOWER_PRIORITY, &HIGHER_PRIORITY];

        sort_by(&mut file_types, cmp_file_types);

        assert_eq!(
            file_types
                .iter()
                .map(|file_type| file_type.id())
                .collect::<Vec<_>>(),
            vec![2, 1]
        );
    }
}
