use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_778: FileType = FileType {
    file_format: &FileFormat {
        id: 778,
        source_type: SourceType::Pronom,
        name: "OpenDocument Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 779,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 780,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 781,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 782,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 783,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_206,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_231,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_599,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_600,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_602,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_603,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_604,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_919,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_920,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_921,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_922,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_923,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 777,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 779,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 780,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 781,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 782,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 783,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_206,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_231,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_599,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_600,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_602,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_603,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_604,
            },
        ],
    },
};
