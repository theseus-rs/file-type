use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1617: FileType = FileType {
    file_format: &FileFormat {
        id: 1_617,
        source_type: SourceType::Pronom,
        name: "JSON Data Interchange Format",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_129,
        }],
    },
};
