use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_696: FileFormat = FileFormat {
    id: 1_495,
    puid: "fmt/696",
    name: "Sibelius",
    extensions: &["sib"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_848,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_849,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_851,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_852,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_854,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_855,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_856,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_857,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_859,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_860,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_861,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_862,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_863,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_864,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_865,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_866,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_848,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_849,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_851,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_852,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_854,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_855,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_856,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_857,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_859,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_860,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_861,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
