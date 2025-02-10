use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1231: FileType = FileType {
    file_format: &FileFormat {
        id: 1_231,
        source_type: SourceType::Pronom,
        name: "OpenDocument Database Format",
        extensions: &["odb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 783,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_206,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_599,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 777,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 778,
            },
        ],
    },
};
