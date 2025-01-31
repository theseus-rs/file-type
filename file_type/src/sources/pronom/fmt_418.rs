use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_418: FileFormat = FileFormat {
    id: 1_200,
    puid: "fmt/418",
    name: "Adobe Illustrator",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x32,
                            0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(16, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x4E, 0x65,
                            0x65, 0x64, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65,
                            0x73, 0x3A,
                        ]),
                        Token::WildcardCountRange(1, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65, 0x74, 0x20,
                            0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74,
                            0x72, 0x61, 0x74, 0x6F, 0x72,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x5F, 0x41, 0x49, 0x33])],
                            &[Token::Literal(&[0x41, 0x5F, 0x41, 0x49, 0x33])],
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
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x32,
                            0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(16, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x53, 0x75,
                            0x70, 0x70, 0x6C, 0x69, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
                            0x63, 0x65, 0x73, 0x3A,
                        ]),
                        Token::WildcardCountRange(1, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65, 0x74, 0x20,
                            0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74,
                            0x72, 0x61, 0x74, 0x6F, 0x72,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x5F, 0x41, 0x49, 0x33])],
                            &[Token::Literal(&[0x41, 0x5F, 0x41, 0x49, 0x33])],
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
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                            0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(16, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x4E, 0x65,
                            0x65, 0x64, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65,
                            0x73, 0x3A,
                        ]),
                        Token::WildcardCountRange(1, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65, 0x74, 0x20,
                            0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74,
                            0x72, 0x61, 0x74, 0x6F, 0x72,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x5F, 0x41, 0x49, 0x33])],
                            &[Token::Literal(&[0x41, 0x5F, 0x41, 0x49, 0x33])],
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
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                            0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(16, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x53, 0x75,
                            0x70, 0x70, 0x6C, 0x69, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
                            0x63, 0x65, 0x73, 0x3A,
                        ]),
                        Token::WildcardCountRange(1, 512),
                        Token::Literal(&[
                            0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65, 0x74, 0x20,
                            0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74,
                            0x72, 0x61, 0x74, 0x6F, 0x72,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x5F, 0x41, 0x49, 0x33])],
                            &[Token::Literal(&[0x41, 0x5F, 0x41, 0x49, 0x33])],
                        ]),
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
                        tokens: &[Token::Literal(&[0xC5, 0xD0, 0xD3, 0xC6])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x33, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(16, 512),
                            Token::Literal(&[
                                0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x53,
                                0x75, 0x70, 0x70, 0x6C, 0x69, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F,
                                0x75, 0x72, 0x63, 0x65, 0x73, 0x3A,
                            ]),
                            Token::WildcardCountRange(1, 512),
                            Token::Literal(&[
                                0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65, 0x74,
                                0x20, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75,
                                0x73, 0x74, 0x72, 0x61, 0x74, 0x6F, 0x72,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x5F, 0x41, 0x49, 0x33])],
                                &[Token::Literal(&[0x41, 0x5F, 0x41, 0x49, 0x33])],
                            ]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_201,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 86,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 331,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 332,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 771,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 773,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
