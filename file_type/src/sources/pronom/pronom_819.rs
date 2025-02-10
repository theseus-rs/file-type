use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_819: FileType = FileType {
    file_format: &FileFormat {
        id: 819,
        source_type: SourceType::Pronom,
        name: "Acrobat PDF/X - Portable Document Format - Exchange 3:2002",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x33,
                            ])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::Variable,
                        offset: None,
                        regex: Regex {
                            tokens: &[
                                Token::Literal(&[
                                    0x2F, 0x47, 0x54, 0x53, 0x5F, 0x50, 0x44, 0x46, 0x58, 0x56,
                                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                                ]),
                                Token::WildcardCountRange(0, 1),
                                Token::Literal(&[
                                    0x28, 0x50, 0x44, 0x46, 0x2F, 0x58, 0x2D, 0x33, 0x3A, 0x32,
                                    0x30, 0x30, 0x32, 0x29,
                                ]),
                            ],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x33,
                            ])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::Variable,
                        offset: None,
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x3C, 0x70, 0x64, 0x66, 0x78, 0x3A, 0x47, 0x54, 0x53, 0x5F, 0x50,
                                0x44, 0x46, 0x58, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E,
                                0x50, 0x44, 0x46, 0x2F, 0x58, 0x2D, 0x33, 0x3A, 0x32, 0x30, 0x30,
                                0x32,
                            ])],
                        },
                    },
                ],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 613,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 614,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 615,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 616,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 617,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 618,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 637,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 869,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_016,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_100,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 791,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 616,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 790,
            },
        ],
    },
};
