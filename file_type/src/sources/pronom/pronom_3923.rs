use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3923: FileType = FileType {
    file_format: &FileFormat {
        id: 3_923,
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
        ],
    },
};
