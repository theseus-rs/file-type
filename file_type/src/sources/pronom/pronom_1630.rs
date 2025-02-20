use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1630: FileType = FileType {
    file_format: &FileFormat {
        id: 1_630,
        source_type: SourceType::Pronom,
        name: "3MF 3D Manufacturing Format",
        extensions: &["3mf"],
        media_types: &["application/vnd.ms-3mfdocument"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_456,
        }],
    },
};
