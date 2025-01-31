use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_493: FileFormat = FileFormat {
    id: 1_280,
    puid: "fmt/493",
    name: "Acrobat PDF/E - Portable Document Format for Engineering PDF/E-1",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E]),
                        Token::Range(&[0x30], &[0x37]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x70, 0x64, 0x66, 0x65, 0x3A, 0x49, 0x53, 0x4F, 0x5F, 0x50, 0x44,
                        0x46, 0x45, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E, 0x50, 0x44,
                        0x46, 0x2F, 0x45, 0x2D, 0x31, 0x3C, 0x2F, 0x70, 0x64, 0x66, 0x65, 0x3A,
                        0x49, 0x53, 0x4F, 0x5F, 0x50, 0x44, 0x46, 0x45, 0x56, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x3E,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 613,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 614,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 615,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 616,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 617,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 618,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 637,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 869,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_016,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 637,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
