use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_658: FileType = FileType {
    file_format: &FileFormat {
        id: 658,
        source_type: SourceType::Pronom,
        name: "Quicktime",
        extensions: &["mov", "qtm"],
        media_types: &["video/quicktime"],
        signatures: &[
            Signature {
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
            Signature {
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
            Signature {
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
            Signature {
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
            Signature {
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
            Signature {
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
            Signature {
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
            Signature {
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 924,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_596,
            },
        ],
    },
};
