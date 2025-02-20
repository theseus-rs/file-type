use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2471: FileType = FileType {
    file_format: &FileFormat {
        id: 2_471,
        source_type: SourceType::Pronom,
        name: "Roxio Label Creator Project File",
        extensions: &["jwl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_472,
            },
        ],
    },
};
