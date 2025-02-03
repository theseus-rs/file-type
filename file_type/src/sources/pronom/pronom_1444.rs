use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1444: FileFormat = FileFormat {
    id: 1_444,
    source_type: SourceType::Pronom,
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
                                0x90, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x04, 0x30, 0x32, 0x32,
                                0x31,
                            ]),
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
                                0x00, 0x90, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x32, 0x32,
                                0x31,
                            ]),
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
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 670,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 676,
        },
    ],
};
