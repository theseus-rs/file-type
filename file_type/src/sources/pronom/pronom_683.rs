use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_683: FileType = FileType {
    file_format: &FileFormat {
        id: 683,
        source_type: SourceType::Pronom,
        name: "Excel 95 Workbook (xls)",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 684,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
