use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_44: FileType = FileType {
    file_format: &FileFormat {
        id: 44,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Template",
        extensions: &["xlt"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 684,
        }],
    },
};
