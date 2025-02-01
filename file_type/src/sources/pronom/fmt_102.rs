use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_102: FileFormat = FileFormat {
    id: 643,
    puid: "fmt/102",
    name: "Extensible Hypertext Markup Language",
    extensions: &["html", "htm"],
    media_types: &["application/xhtml+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x68, 0x74,
                        0x6D, 0x6C, 0x20, 0x50, 0x55, 0x42, 0x4C, 0x49, 0x43, 0x20, 0x22, 0x2D,
                        0x2F, 0x2F, 0x57, 0x33, 0x43, 0x2F, 0x2F, 0x44, 0x54, 0x44, 0x20, 0x58,
                        0x48, 0x54, 0x4D, 0x4C, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x68, 0x74, 0x6D, 0x6C, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D,
                            0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E,
                            0x77, 0x33, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x31, 0x39, 0x39, 0x39, 0x2F,
                            0x78, 0x68, 0x74, 0x6D, 0x6C, 0x22,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x3C, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x3E]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x3C, 0x2F, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x3E]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 310,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 777,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_029,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_158,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_269,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_270,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_670,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_099,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_173,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 645,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 644,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
