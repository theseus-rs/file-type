use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_359: FileType = FileType {
    file_format: &FileFormat {
        id: 359,
        source_type: SourceType::Pronom,
        name: "Microsoft Project",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 360,
        }],
    },
};
