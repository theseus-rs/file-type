use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_20: FileFormat = FileFormat {
    id: 20,
    source_type: SourceType::Pronom,
    name: "dBASE Database",
    extensions: &["dbf"],
    media_types: &["application/dbase"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x04]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0x43]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0x63]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0xCB]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4D])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0x7B]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4D])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0x8B]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4D])],
                            &[Token::Literal(&[0x4E])],
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
                        Token::Literal(&[0x8E]),
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
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4D])],
                            &[Token::Literal(&[0x4E])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
