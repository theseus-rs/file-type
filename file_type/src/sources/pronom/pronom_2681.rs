use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2681: FileType = FileType {
    file_format: &FileFormat {
        id: 2_681,
        source_type: SourceType::Pronom,
        name: "PPTX Strict OOXML Presentation",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 941,
        }],
    },
};
