use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2599: FileType = FileType {
    file_format: &FileFormat {
        id: 2_599,
        source_type: SourceType::Pronom,
        name: "OpenDocument Database Format",
        extensions: &["odb"],
        media_types: &["application/vnd.oasis.opendocument.base"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_231,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 778,
            },
        ],
    },
};
