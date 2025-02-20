use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_682: FileType = FileType {
    file_format: &FileFormat {
        id: 682,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel 5.0/95 Workbook (xls)",
        extensions: &["xlw", "xls"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 684,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 680,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 681,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
