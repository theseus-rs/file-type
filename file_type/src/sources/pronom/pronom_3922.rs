use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3922: FileType = FileType {
    file_format: &FileFormat {
        id: 3_922,
        source_type: SourceType::Pronom,
        name: "OpenDocument Database",
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
                id: 783,
            },
        ],
    },
};
