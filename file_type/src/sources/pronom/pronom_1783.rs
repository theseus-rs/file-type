use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1783: FileType = FileType {
    file_format: &FileFormat {
        id: 1_783,
        source_type: SourceType::Pronom,
        name: "3DS Max",
        extensions: &["max", "chr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_292,
        }],
    },
};
