use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_481: FileFormat = FileFormat {
    id: 1_268,
    puid: "fmt/481",
    name: "Acrobat PDF/A - Portable Document Format",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E]),
                            Token::Range(&[0x30], &[0x37]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x70, 0x64, 0x66, 0x61, 0x69,
                                0x64, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E,
                                0x61, 0x69, 0x69, 0x6D, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x70, 0x64,
                                0x66, 0x61, 0x2F, 0x6E, 0x73, 0x2F, 0x69, 0x64,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61, 0x72, 0x74,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3D, 0x22])],
                                &[Token::Literal(&[0x3D, 0x27])],
                                &[Token::Literal(&[0x3E])],
                            ]),
                            Token::Literal(&[0x33]),
                            Token::Any(&[
                                &[Token::Literal(&[0x22])],
                                &[Token::Literal(&[0x27])],
                                &[Token::Literal(&[
                                    0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70,
                                    0x61, 0x72, 0x74, 0x3E,
                                ])],
                            ]),
                            Token::WildcardCountRange(0, 120),
                            Token::Literal(&[
                                0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63, 0x6F, 0x6E, 0x66,
                                0x6F, 0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3E])],
                                &[Token::Literal(&[0x3D, 0x22])],
                                &[Token::Literal(&[0x3D, 0x27])],
                            ]),
                            Token::Literal(&[0x55]),
                            Token::Any(&[
                                &[Token::Literal(&[0x22])],
                                &[Token::Literal(&[0x27])],
                                &[Token::Literal(&[
                                    0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63,
                                    0x6F, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65,
                                    0x3E,
                                ])],
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
                        tokens: &[
                            Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E]),
                            Token::Range(&[0x30], &[0x37]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x70, 0x64, 0x66, 0x61, 0x69,
                                0x64, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E,
                                0x61, 0x69, 0x69, 0x6D, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x70, 0x64,
                                0x66, 0x61, 0x2F, 0x6E, 0x73, 0x2F, 0x69, 0x64,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63, 0x6F, 0x6E, 0x66,
                                0x6F, 0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3E])],
                                &[Token::Literal(&[0x3D, 0x22])],
                                &[Token::Literal(&[0x3D, 0x27])],
                            ]),
                            Token::Literal(&[0x55]),
                            Token::Any(&[
                                &[Token::Literal(&[0x22])],
                                &[Token::Literal(&[0x27])],
                                &[Token::Literal(&[
                                    0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63,
                                    0x6F, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65,
                                    0x3E,
                                ])],
                            ]),
                            Token::WildcardCountRange(0, 120),
                            Token::Literal(&[
                                0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61, 0x72, 0x74,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3D, 0x22])],
                                &[Token::Literal(&[0x3D, 0x27])],
                                &[Token::Literal(&[0x3E])],
                            ]),
                            Token::Literal(&[0x33]),
                            Token::Any(&[
                                &[Token::Literal(&[0x22])],
                                &[Token::Literal(&[0x27])],
                                &[Token::Literal(&[
                                    0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70,
                                    0x61, 0x72, 0x74, 0x3E,
                                ])],
                            ]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 613,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 614,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 615,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 616,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 617,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 618,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 637,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 869,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_016,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_016,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
