use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_684: FileType = FileType {
    file_format: &FileFormat {
        id: 684,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel 97-2003 Workbook (xls)",
        extensions: &["xls", "xlw"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 44,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_706,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 682,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 685,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 683,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
