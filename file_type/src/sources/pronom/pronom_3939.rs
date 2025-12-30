use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3939: FileType = FileType {
    file_format: &FileFormat {
        id: 3_939,
        source_type: SourceType::Pronom,
        name: "DaVinci Resolve Project File",
        extensions: &["drp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 3_938,
        }],
    },
};
