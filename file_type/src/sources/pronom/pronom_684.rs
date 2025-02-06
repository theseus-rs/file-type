use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_684: FileFormat = FileFormat {
    id: 684,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel 97 Workbook (xls)",
    extensions: &["xls", "xlw"],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[Signature {
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
};
