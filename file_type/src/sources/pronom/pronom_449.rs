use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_449: FileType = FileType {
    file_format: &FileFormat {
        id: 449,
        source_type: SourceType::Pronom,
        name: "Micrografx Designer",
        extensions: &["drw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 447,
        }],
    },
};
