use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_688: FileType = FileType {
    file_format: &FileFormat {
        id: 688,
        source_type: SourceType::Pronom,
        name: "Microsoft Word Document",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]),
                            Token::WildcardCount(20),
                            Token::Literal(&[0xFE, 0xFF]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x57, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x64, 0x00, 0x44, 0x00, 0x6F,
                                0x00, 0x63, 0x00, 0x75, 0x00, 0x6D, 0x00, 0x65, 0x00, 0x6E, 0x00,
                                0x74, 0x00,
                            ]),
                            Token::WildcardCount(42),
                            Token::Literal(&[0x02]),
                            Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x57,
                                0x6F, 0x72, 0x64, 0x20,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x36, 0x2E, 0x30])],
                                &[Token::Literal(&[
                                    0x66, 0x6F, 0x72, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                                    0x73, 0x20, 0x39, 0x35,
                                ])],
                                &[Token::Literal(&[
                                    0x36, 0x2E, 0x30, 0x2D, 0x44, 0x6F, 0x6B, 0x75, 0x6D, 0x65,
                                    0x6E, 0x74,
                                ])],
                            ]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_401,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 690,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_553,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 734,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
