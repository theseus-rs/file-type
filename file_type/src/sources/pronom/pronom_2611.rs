use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2611: FileType = FileType {
    file_format: &FileFormat {
        id: 2_611,
        source_type: SourceType::Pronom,
        name: "MacBinary",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_612,
        }],
    },
};
