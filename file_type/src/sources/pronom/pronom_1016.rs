use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1016: FileFormat = FileFormat {
    id: 1_016,
    source_type: SourceType::Pronom,
    name: "Acrobat PDF 1.7 - Portable Document Format",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x37,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x25, 0x45, 0x4F]),
                        Token::Any(&[
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x46, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D])],
                            &[Token::Literal(&[0x46, 0x0D, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D, 0x00])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 770,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 787,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 788,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 789,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 790,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 791,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 818,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 819,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_100,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_263,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_264,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_265,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_266,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_267,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_268,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_275,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_276,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_277,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_278,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_279,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_280,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_272,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 869,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 637,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_263,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_264,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_265,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_266,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_267,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_268,
        },
    ],
};
