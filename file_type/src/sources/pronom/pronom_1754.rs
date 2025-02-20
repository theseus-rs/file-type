use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1754: FileType = FileType {
    file_format: &FileFormat {
        id: 1_754,
        source_type: SourceType::Pronom,
        name: "WordPerfect",
        extensions: &["wp4", "wpd"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 736,
        }],
    },
};
