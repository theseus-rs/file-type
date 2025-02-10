use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_76: FileType = FileType {
    file_format: &FileFormat {
        id: 76,
        source_type: SourceType::Pronom,
        name: "Microsoft Word Document Template",
        extensions: &["dot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_554,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 690,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_401,
            },
        ],
    },
};
