use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_158: FileFormat = FileFormat {
    id: 819,
    puid: "fmt/158",
    name: "Acrobat PDF/X - Portable Document Format - Exchange 3:2002",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[
        InternalSignature {
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
                                0x2F, 0x47, 0x54, 0x53, 0x5F, 0x50, 0x44, 0x46, 0x58, 0x56, 0x65,
                                0x72, 0x73, 0x69, 0x6F, 0x6E,
                            ]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[
                                0x28, 0x50, 0x44, 0x46, 0x2F, 0x58, 0x2D, 0x33, 0x3A, 0x32, 0x30,
                                0x30, 0x32, 0x29,
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
                            0x3C, 0x70, 0x64, 0x66, 0x78, 0x3A, 0x47, 0x54, 0x53, 0x5F, 0x50, 0x44,
                            0x46, 0x58, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E, 0x50, 0x44,
                            0x46, 0x2F, 0x58, 0x2D, 0x33, 0x3A, 0x32, 0x30, 0x30, 0x32,
                        ])],
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
            id: 1_100,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 791,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 616,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
        RelatedFormat {
            id: 790,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
