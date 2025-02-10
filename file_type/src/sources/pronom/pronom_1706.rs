use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1706: FileType = FileType {
    file_format: &FileFormat {
        id: 1_706,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Spreadsheet",
        extensions: &["xlr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 684,
        }],
    },
};
