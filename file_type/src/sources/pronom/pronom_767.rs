use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_767: FileType = FileType {
    file_format: &FileFormat {
        id: 767,
        source_type: SourceType::Pronom,
        name: "OLE2 Compound Document Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 76,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 133,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 134,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 135,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 165,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 376,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 682,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 684,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 685,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 688,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 690,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 822,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_229,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_230,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_553,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_554,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_471,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_472,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 134,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 135,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 682,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 683,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 684,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 685,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 688,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 690,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 857,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 858,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 859,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 861,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 862,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_553,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_554,
            },
        ],
    },
};
