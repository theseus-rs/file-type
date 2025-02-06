use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1092: FileFormat = FileFormat {
    id: 1_092,
    source_type: SourceType::Pronom,
    name: "MPEG 1/2 Audio Layer I",
    extensions: &["mp1"],
    media_types: &["audio/mpeg"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xF6]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF6]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF6]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF6]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xF7]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF7]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF7]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xF7]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xFE]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFE]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFE]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFE]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::Range(&[0x10], &[0xEB]),
                        Token::WildcardCountRange(29, 769),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 923,
    }],
};
