use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_286: FileType = FileType {
    file_format: &FileFormat {
        id: 286,
        source_type: SourceType::Pronom,
        name: "WordStar for Windows Document",
        extensions: &["wsd", "ws", "wsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 380,
        }],
    },
};
