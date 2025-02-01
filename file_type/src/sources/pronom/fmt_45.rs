use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_45: FileFormat = FileFormat {
    id: 626,
    puid: "fmt/45",
    name: "Rich Text Format",
    extensions: &["rtf"],
    media_types: &["application/rtf", "text/rtf"],
    internal_signatures: &[InternalSignature {
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
            id: 631,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 633,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 753,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_101,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 627,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
