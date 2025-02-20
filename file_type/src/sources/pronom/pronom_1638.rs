use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1638: FileType = FileType {
    file_format: &FileFormat {
        id: 1_638,
        source_type: SourceType::Pronom,
        name: "Quattro Pro Spreadsheet",
        extensions: &["qpw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_637,
        }],
    },
};
