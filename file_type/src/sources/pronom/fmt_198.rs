use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_198: FileFormat = FileFormat {
    id: 923,
    puid: "fmt/198",
    name: "MPEG Audio Stream Layer II",
    extensions: &["mp2", "mpw", "mpa"],
    media_types: &["audio/mpeg"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xFC]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFC]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFC]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFC]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
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
                        Token::Literal(&[0xFF, 0xFD]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFD]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFD]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xFD]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
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
                        Token::Literal(&[0xFF, 0xF4]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF4]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF4]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF4]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
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
                        Token::Literal(&[0xFF, 0xF5]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF5]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF5]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                        Token::Literal(&[0xFF, 0xF5]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(45, 1_726),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xFC]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFC]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFC]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFC]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFC]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(7, 500),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::WildcardCountRange(36, 1_426),
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
                        tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xFD]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFD]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFD]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFD]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xFD]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(7, 500),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::WildcardCountRange(36, 1_426),
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
                        tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xF4]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF4]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF4]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF4]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF4]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(7, 500),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::WildcardCountRange(36, 1_426),
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
                        tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xF5]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF5]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF5]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF5]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(45, 1_726),
                            Token::Literal(&[0xFF, 0xF5]),
                            Token::Range(&[0x10], &[0xEB]),
                            Token::WildcardCountRange(7, 500),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::WildcardCountRange(36, 1_426),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 425,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_092,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
