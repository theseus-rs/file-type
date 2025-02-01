use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_123: FileFormat = FileFormat {
    id: 332,
    puid: "fmt/123",
    name: "Encapsulated PostScript File Format",
    extensions: &["eps", "epsf"],
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
                        ]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0x45, 0x50, 0x53, 0x46, 0x2D, 0x32]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x0A]),
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
            id: 49,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_199,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_200,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_201,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_202,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_204,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_205,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_345,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 138,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 771,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 772,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 773,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_073,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 331,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_740,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 86,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
