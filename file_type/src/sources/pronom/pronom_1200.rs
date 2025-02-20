use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1200: FileType = FileType {
    file_format: &FileFormat {
        id: 1_200,
        source_type: SourceType::Pronom,
        name: "Adobe Illustrator",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x32, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(16, 512),
                            Token::Literal(&[
                                0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x4E,
                                0x65, 0x65, 0x64, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
                                0x63, 0x65, 0x73, 0x3A,
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
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x32, 0x2E, 0x30,
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
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x33, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(16, 512),
                            Token::Literal(&[
                                0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x4E,
                                0x65, 0x65, 0x64, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
                                0x63, 0x65, 0x73, 0x3A,
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
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
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
                }],
            },
            Signature {
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
                                    0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65,
                                    0x2D, 0x33, 0x2E, 0x30,
                                ]),
                                Token::WildcardCountRange(16, 512),
                                Token::Literal(&[
                                    0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                                    0x53, 0x75, 0x70, 0x70, 0x6C, 0x69, 0x65, 0x64, 0x52, 0x65,
                                    0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x73, 0x3A,
                                ]),
                                Token::WildcardCountRange(1, 512),
                                Token::Literal(&[
                                    0x25, 0x25, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x63, 0x73, 0x65,
                                    0x74, 0x20, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C,
                                    0x6C, 0x75, 0x73, 0x74, 0x72, 0x61, 0x74, 0x6F, 0x72,
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_201,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 86,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 331,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 332,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 771,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 773,
            },
        ],
    },
};
