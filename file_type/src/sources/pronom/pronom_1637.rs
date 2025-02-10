use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1637: FileType = FileType {
    file_format: &FileFormat {
        id: 1_637,
        source_type: SourceType::Pronom,
        name: "Quattro Pro Spreadsheet",
        extensions: &["wb3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_636,
            },
        ],
    },
};
