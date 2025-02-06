use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_626: FileFormat = FileFormat {
    id: 626,
    source_type: SourceType::Pronom,
    name: "Rich Text Format",
    extensions: &["rtf"],
    media_types: &["application/rtf", "text/rtf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x7B, 0x5C, 0x72, 0x74]),
                    Token::Any(&[&[Token::Literal(&[0x66])], &[Token::Literal(&[0x66, 0x31])]]),
                    Token::Literal(&[0x5C]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61, 0x6E, 0x73, 0x69])],
                        &[Token::Literal(&[0x6D, 0x61, 0x63])],
                        &[Token::Literal(&[0x70, 0x63])],
                        &[Token::Literal(&[0x70, 0x63, 0x61])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 631,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 633,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 753,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_101,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 627,
        },
    ],
};
