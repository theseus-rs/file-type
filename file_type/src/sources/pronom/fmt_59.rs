use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_59: FileFormat = FileFormat {
    id: 682,
    puid: "fmt/59",
    name: "Microsoft Excel 5.0/95 Workbook (xls)",
    extensions: &["xlw", "xls"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 684,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 680,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 681,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
