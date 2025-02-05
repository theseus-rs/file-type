use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_683: FileFormat = FileFormat {
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
};
