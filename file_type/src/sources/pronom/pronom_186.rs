use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_186: FileType = FileType {
    file_format: &FileFormat {
        id: 186,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Workspace File",
        extensions: &["xlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_712,
        }],
    },
};
