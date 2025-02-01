use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_111: FileFormat = FileFormat {
    id: 767,
    puid: "fmt/111",
    name: "OLE2 Compound Document Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0xFE, 0xFF]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 76,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 133,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 134,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 135,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 165,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 376,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 682,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 684,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 685,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 688,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 690,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 822,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_229,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_230,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_553,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_554,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_471,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_472,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 134,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 135,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 682,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 683,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 684,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 685,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 688,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 690,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 857,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 858,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 859,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 861,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 862,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_553,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_554,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
