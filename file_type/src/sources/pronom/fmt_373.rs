use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_373: FileFormat = FileFormat {
    id: 1_120,
    puid: "fmt/373",
    name: "FoxPro Database",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF5]),
                    Token::SingleWildcard,
                    Token::Range(&[0x01], &[0x0C]),
                    Token::Range(&[0x01], &[0x1F]),
                    Token::WildcardCount(28),
                    Token::Any(&[
                        &[Token::Range(&[0x41], &[0x5A])],
                        &[Token::Range(&[0x61], &[0x7A])],
                    ]),
                    Token::WildcardCount(10),
                    Token::Any(&[
                        &[Token::Literal(&[0x42])],
                        &[Token::Literal(&[0x43])],
                        &[Token::Literal(&[0x44])],
                        &[Token::Literal(&[0x49])],
                        &[Token::Literal(&[0x4C])],
                        &[Token::Literal(&[0x4D])],
                        &[Token::Literal(&[0x4E])],
                        &[Token::Literal(&[0x50])],
                        &[Token::Literal(&[0x54])],
                        &[Token::Literal(&[0x59])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_123,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_128,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
