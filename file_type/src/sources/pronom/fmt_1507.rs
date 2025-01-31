use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1507: FileFormat = FileFormat {
    id: 2_330,
    puid: "fmt/1507",
    name: "Exchangeable Image File Format (Compressed)",
    extensions: &["jpg", "jpeg"],
    media_types: &["image/jpeg"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xD8, 0xFF, 0xE1]),
                            Token::WildcardCount(2),
                            Token::Literal(&[
                                0x45, 0x78, 0x69, 0x66, 0x00, 0x00, 0x4D, 0x4D, 0x00, 0x2A,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x90, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x04, 0x30, 0x32, 0x33,
                            ]),
                            Token::Range(&[0x30], &[0x39]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xD9])],
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
                        tokens: &[
                            Token::Literal(&[0xFF, 0xD8, 0xFF, 0xE1]),
                            Token::WildcardCount(2),
                            Token::Literal(&[
                                0x45, 0x78, 0x69, 0x66, 0x00, 0x00, 0x49, 0x49, 0x2A, 0x00,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x00, 0x90, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x32, 0x33,
                            ]),
                            Token::Range(&[0x30], &[0x39]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xD9])],
                    },
                },
            ],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 670,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
