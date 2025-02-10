use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2115: FileType = FileType {
    file_format: &FileFormat {
        id: 2_115,
        source_type: SourceType::Pronom,
        name: "Calendar Creator File",
        extensions: &["cc5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_116,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_114,
            },
        ],
    },
};
