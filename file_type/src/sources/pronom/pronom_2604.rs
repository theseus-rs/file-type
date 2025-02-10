use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2604: FileType = FileType {
    file_format: &FileFormat {
        id: 2_604,
        source_type: SourceType::Pronom,
        name: "OpenDocument Text",
        extensions: &["odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_033,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 778,
            },
        ],
    },
};
