use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_363: FileFormat = FileFormat {
    id: 1_110,
    puid: "fmt/363",
    name: "SEG Y Data Exchange Format",
    extensions: &["segy"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
                            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(3_200),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(15),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(3),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(2),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x00]), Token::Range(&[0x01], &[0x08])],
                            ]),
                        ],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
                            0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(3_600),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(15),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(3),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(2),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x00]), Token::Range(&[0x01], &[0x08])],
                            ]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[],
};
