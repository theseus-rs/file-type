use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2602: FileType = FileType {
    file_format: &FileFormat {
        id: 2_602,
        source_type: SourceType::Pronom,
        name: "OpenDocument Presentation",
        extensions: &["odp"],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_035,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 778,
            },
        ],
    },
};
