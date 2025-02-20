use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_542: FileType = FileType {
    file_format: &FileFormat {
        id: 542,
        source_type: SourceType::Pronom,
        name: "WordStar for MS-DOS Document",
        extensions: &["ws3", "ws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 378,
        }],
    },
};
