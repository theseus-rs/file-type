use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1311: FileFormat = FileFormat {
    id: 2_129,
    puid: "fmt/1311",
    name: "Tweet JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x7B]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x22, 0x69, 0x64, 0x5F, 0x73, 0x74, 0x72, 0x22, 0x3A]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x22, 0x72, 0x65, 0x74, 0x77, 0x65, 0x65, 0x74, 0x65, 0x64, 0x22,
                                0x3A,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7D, 0x0A])],
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
                            Token::Literal(&[0x7B]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x22, 0x69, 0x64, 0x5F, 0x73, 0x74, 0x72, 0x22, 0x3A]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x22, 0x72, 0x65, 0x74, 0x77, 0x65, 0x65, 0x74, 0x65, 0x64, 0x22,
                                0x3A,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7D])],
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
                            Token::Literal(&[0x7B]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x22, 0x69, 0x64, 0x5F, 0x73, 0x74, 0x72, 0x22, 0x3A]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x22, 0x72, 0x65, 0x74, 0x77, 0x65, 0x65, 0x74, 0x65, 0x64, 0x22,
                                0x3A,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7D, 0x0D, 0x0A])],
                    },
                },
            ],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 1_617,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
