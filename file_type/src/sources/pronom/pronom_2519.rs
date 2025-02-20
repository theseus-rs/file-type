use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2519: FileType = FileType {
    file_format: &FileFormat {
        id: 2_519,
        source_type: SourceType::Pronom,
        name: "EndNote Compressed Library",
        extensions: &["enlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_521,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_070,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_518,
            },
        ],
    },
};
