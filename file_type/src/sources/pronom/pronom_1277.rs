use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1277: FileFormat = FileFormat {
    id: 1_277,
    source_type: SourceType::Pronom,
    name: "Acrobat PDF/X - Portable Document Format - Exchange PDF/X-5g",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    signatures: &[Signature {
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
                        0x3C, 0x70, 0x64, 0x66, 0x78, 0x69, 0x64, 0x3A, 0x47, 0x54, 0x53, 0x5F,
                        0x50, 0x44, 0x46, 0x58, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E,
                        0x50, 0x44, 0x46, 0x2F, 0x58, 0x2D, 0x35, 0x67, 0x3C, 0x2F, 0x70, 0x64,
                        0x66, 0x78, 0x69, 0x64, 0x3A, 0x47, 0x54, 0x53, 0x5F, 0x50, 0x44, 0x46,
                        0x58, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 613,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 614,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 615,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 616,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 617,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 618,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 637,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 869,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_016,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 637,
        },
    ],
};
