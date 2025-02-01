use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_61: FileFormat = FileFormat {
    id: 684,
    puid: "fmt/61",
    name: "Microsoft Excel 97 Workbook (xls)",
    extensions: &["xls", "xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(512),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x08]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x06, 0x05, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 44,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_706,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 682,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 685,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 683,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
