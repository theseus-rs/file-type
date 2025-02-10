use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_378: FileType = FileType {
    file_format: &FileFormat {
        id: 378,
        source_type: SourceType::Pronom,
        name: "WordStar for MS-DOS Document",
        extensions: &["ws", "ws4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 285,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 542,
            },
        ],
    },
};
