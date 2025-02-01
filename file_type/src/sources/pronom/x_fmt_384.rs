use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_384: FileFormat = FileFormat {
    id: 658,
    puid: "x-fmt/384",
    name: "Quicktime",
    extensions: &["mov", "qtm"],
    media_types: &["video/quicktime"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x6D, 0x64, 0x61, 0x74]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x63, 0x6D, 0x6F, 0x76]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x74, 0x79, 0x70, 0x71, 0x74, 0x20, 0x20,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x70, 0x6E, 0x6F, 0x74]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x73, 0x6B, 0x69, 0x70]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x66, 0x72, 0x65, 0x65]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x77, 0x69, 0x64, 0x65]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Any(&[
                            &[Token::Literal(&[0x6D, 0x76, 0x68, 0x64])],
                            &[Token::Literal(&[0x63, 0x6D, 0x6F, 0x76])],
                            &[Token::Literal(&[0x72, 0x6D, 0x72, 0x61])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 924,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_596,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
