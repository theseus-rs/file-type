use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3916: FileType = FileType {
    file_format: &FileFormat {
        id: 3_916,
        source_type: SourceType::Pronom,
        name: "Android Package File",
        extensions: &["apk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        }],
    },
};
