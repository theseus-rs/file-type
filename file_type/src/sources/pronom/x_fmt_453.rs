use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_453: FileFormat = FileFormat {
    id: 869,
    puid: "x-fmt/453",
    name: "TrueType Font",
    extensions: &["ttf"],
    media_types: &["font/ttf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(12),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x53, 0x2F, 0x32]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x63, 0x6D, 0x61, 0x70]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x67, 0x6C, 0x79, 0x66]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x68, 0x65, 0x61, 0x64]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x68, 0x68, 0x65, 0x61]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x68, 0x6D, 0x74, 0x78]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x6C, 0x6F, 0x63, 0x61]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x6D, 0x61, 0x78, 0x70]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x6E, 0x61, 0x6D, 0x65]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x70, 0x6F, 0x73, 0x74]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 613,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 614,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 615,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 616,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 617,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 618,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 637,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 770,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 787,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 788,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 789,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 790,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 791,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 818,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 819,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_016,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_100,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_263,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_264,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_265,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_266,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_267,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_268,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_275,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_276,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_277,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_278,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_279,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_280,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_412,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
