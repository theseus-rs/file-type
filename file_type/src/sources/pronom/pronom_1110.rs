use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1110: FileFormat = FileFormat {
    id: 1_110,
    source_type: SourceType::Pronom,
    name: "SEG Y Data Exchange Format",
    extensions: &["segy"],
    media_types: &[],
    signatures: &[
        Signature {
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
        Signature {
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
