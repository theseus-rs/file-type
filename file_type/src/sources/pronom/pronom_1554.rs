use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1554: FileType = FileType {
    file_format: &FileFormat {
        id: 1_554,
        source_type: SourceType::Pronom,
        name: "Microsoft Word Document Template (Password Protected)",
        extensions: &["dot"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 76,
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
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_553,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
