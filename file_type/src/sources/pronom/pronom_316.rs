use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_316: FileType = FileType {
    file_format: &FileFormat {
        id: 316,
        source_type: SourceType::Pronom,
        name: "Cascading Style Sheet",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 635,
        }],
    },
};
