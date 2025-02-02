use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_506: FileFormat = FileFormat {
    id: 506,
    source_type: SourceType::Pronom,
    name: "Microsoft FoxPro Memo",
    extensions: &["fpt", "frt", "vct", "pjt"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::NotLiteral(&[0x00]),
                        Token::WildcardCount(504),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Range(&[0x00], &[0x02]),
                        Token::WildcardCount(4),
                        Token::NotLiteral(&[0x00]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::NotLiteral(&[0x00]),
                        Token::WildcardCount(520),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Range(&[0x00], &[0x02]),
                        Token::WildcardCount(4),
                        Token::NotLiteral(&[0x00]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::NotLiteral(&[0x00]),
                        Token::WildcardCount(504),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Range(&[0x00], &[0x02]),
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                        ]),
                        Token::NotLiteral(&[0x00]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 1_131,
    }],
};
