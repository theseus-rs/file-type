use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_135: FileFormat = FileFormat {
    id: 778,
    puid: "fmt/135",
    name: "OpenDocument Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                    Token::WildcardCount(26),
                    Token::Literal(&[
                        0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70, 0x6C,
                        0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E, 0x64, 0x2E,
                        0x6F, 0x61, 0x73, 0x69, 0x73, 0x2E, 0x6F, 0x70, 0x65, 0x6E, 0x64, 0x6F,
                        0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x2E,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 779,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 780,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 781,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 782,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 783,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_206,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_231,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_599,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_600,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_602,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_603,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_604,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 777,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 779,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 780,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 781,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 782,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 783,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_206,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_231,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_599,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_600,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_602,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_603,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_604,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
