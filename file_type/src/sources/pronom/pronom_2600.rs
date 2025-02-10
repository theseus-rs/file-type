use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2600: FileType = FileType {
    file_format: &FileFormat {
        id: 2_600,
        source_type: SourceType::Pronom,
        name: "OpenDocument Graphics",
        extensions: &["odg"],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_039,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 778,
            },
        ],
    },
};
