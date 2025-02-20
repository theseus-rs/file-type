use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_380: FileType = FileType {
    file_format: &FileFormat {
        id: 380,
        source_type: SourceType::Pronom,
        name: "WordStar for Windows Document",
        extensions: &["ws", "wsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 286,
        }],
    },
};
