use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_473: FileFormat = FileFormat {
    id: 1_260,
    puid: "fmt/473",
    name: "Microsoft Office Owner File",
    extensions: &["doc", "docx"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x06]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x06, 0x00]),
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
                        Token::Literal(&[0x07]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x07, 0x00]),
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
                        Token::Literal(&[0x08]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x08, 0x00]),
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
                        Token::Literal(&[0x09]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x09, 0x00]),
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
                        Token::Literal(&[0x0A]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0A, 0x00]),
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
                        Token::Literal(&[0x0B]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0B, 0x00]),
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
                        Token::Literal(&[0x0C]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0C, 0x00]),
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
                        Token::Literal(&[0x0D]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0D, 0x00]),
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
                        Token::Literal(&[0x0E]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0E, 0x00]),
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
                        Token::Literal(&[0x0F]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x0F, 0x00]),
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
                        Token::Literal(&[0x01]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x01, 0x00]),
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
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x02, 0x00]),
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
                        Token::Literal(&[0x03]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x03, 0x00]),
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
                        Token::Literal(&[0x04]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x04, 0x00]),
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
                        Token::Literal(&[0x05]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x05, 0x00]),
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
                        Token::Literal(&[0x14]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x14, 0x00]),
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
                        Token::Literal(&[0x10]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x10, 0x00]),
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
                        Token::Literal(&[0x11]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x11, 0x00]),
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
                        Token::Literal(&[0x12]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x12, 0x00]),
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
                        Token::Literal(&[0x13]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x13, 0x00]),
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
                        Token::Literal(&[0x15]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x15, 0x00]),
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
                        Token::Literal(&[0x16]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x16, 0x00]),
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
                        Token::Literal(&[0x17]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x17, 0x00]),
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
                        Token::Literal(&[0x18]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x18, 0x00]),
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
                        Token::Literal(&[0x19]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0x00, 0x19, 0x00]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 2_870,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
