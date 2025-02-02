use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_682: FileFormat = FileFormat {
    id: 682,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel 5.0/95 Workbook (xls)",
    extensions: &["xlw", "xls"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
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
};
