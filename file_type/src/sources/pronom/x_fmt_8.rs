use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_8: FileFormat = FileFormat {
    id: 18,
    puid: "x-fmt/8",
    name: "dBASE Database",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x02]),
                        Token::WildcardCount(2),
                        Token::Range(&[0x01], &[0x1C]),
                        Token::Range(&[0x01], &[0x1F]),
                        Token::SingleWildcard,
                        Token::SingleWildcard,
                        Token::Range(&[0x00], &[0x03]),
                        Token::Any(&[
                            &[Token::Range(&[0x41], &[0x5A])],
                            &[Token::Range(&[0x61], &[0x7A])],
                        ]),
                        Token::WildcardCount(10),
                        Token::Any(&[
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x4E])],
                            &[Token::Literal(&[0x4C])],
                        ]),
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
                        Token::Literal(&[0x02]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::SingleWildcard,
                        Token::Range(&[0x00], &[0x03]),
                        Token::Any(&[
                            &[Token::Range(&[0x41], &[0x5A])],
                            &[Token::Range(&[0x61], &[0x7A])],
                        ]),
                        Token::WildcardCount(10),
                        Token::Any(&[
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x4E])],
                            &[Token::Literal(&[0x4C])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
