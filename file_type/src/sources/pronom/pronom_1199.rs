use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1199: FileType = FileType {
    file_format: &FileFormat {
        id: 1_199,
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
                            Token::WildcardCountRange(1, 16),
                            Token::Literal(&[0x25, 0x25, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3A])],
                                &[Token::Literal(&[0x3A, 0x20])],
                            ]),
                            Token::Literal(&[
                                0x41, 0x64, 0x6F, 0x62, 0x65, 0x20, 0x49, 0x6C, 0x6C, 0x75, 0x73,
                                0x74, 0x72, 0x61, 0x74, 0x6F, 0x72,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x28, 0x54, 0x4D, 0x29, 0x20, 0x38, 0x38])],
                                &[Token::Literal(&[0x20, 0x38, 0x38, 0x28, 0x54, 0x4D, 0x29])],
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
                                0x32, 0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x31, 0x2E,
                                0x32,
                            ]),
                            Token::WildcardCountRange(10, 2_096),
                            Token::Literal(&[
                                0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x50,
                                0x72, 0x6F, 0x63, 0x53, 0x65, 0x74, 0x73, 0x3A, 0x41, 0x64, 0x6F,
                                0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72, 0x61,
                                0x74, 0x6F, 0x72, 0x5F, 0x38, 0x38,
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
                                0x32, 0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x31, 0x2E,
                                0x32,
                            ]),
                            Token::WildcardCountRange(10, 2_096),
                            Token::Literal(&[
                                0x25, 0x25, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x50,
                                0x72, 0x6F, 0x63, 0x53, 0x65, 0x74, 0x73, 0x3A, 0x20, 0x41, 0x64,
                                0x6F, 0x62, 0x65, 0x5F, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72,
                                0x61, 0x74, 0x6F, 0x72,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x5F, 0x38, 0x38])],
                                &[Token::Literal(&[0x38, 0x38])],
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
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
