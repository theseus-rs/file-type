use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_332: FileFormat = FileFormat {
    id: 332,
    source_type: SourceType::Pronom,
    name: "Encapsulated PostScript File Format",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x32,
                        ]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0x45, 0x50, 0x53, 0x46, 0x2D, 0x32]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x0A]),
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
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x32, 0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x32, 0x2E,
                                0x30,
                            ]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A, 0x0D])],
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
            id: 49,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_199,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_200,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_201,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_202,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_204,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_205,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_345,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 138,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 771,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 772,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 773,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_073,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 331,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_740,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 86,
        },
    ],
};
