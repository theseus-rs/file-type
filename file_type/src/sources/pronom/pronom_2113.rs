use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2113: FileType = FileType {
    file_format: &FileFormat {
        id: 2_113,
        source_type: SourceType::Pronom,
        name: "Calendar Creator Event",
        extensions: &["ce3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 202,
        }],
    },
};
