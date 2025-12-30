use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3919: FileType = FileType {
    file_format: &FileFormat {
        id: 3_919,
        source_type: SourceType::Pronom,
        name: "OpenDocument Text",
        extensions: &["odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 778,
        }],
    },
};
