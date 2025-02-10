use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_281: FileType = FileType {
    file_format: &FileFormat {
        id: 281,
        source_type: SourceType::Pronom,
        name: "WordPerfect for Windows Document",
        extensions: &["w52", "wp", "wpd", "wp5"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 75,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 737,
            },
        ],
    },
};
