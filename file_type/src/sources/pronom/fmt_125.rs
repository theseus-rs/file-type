use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_125: FileFormat = FileFormat {
    id: 134,
    puid: "fmt/125",
    name: "Microsoft Powerpoint Presentation",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0xFE, 0xFF]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x00, 0x6F, 0x00, 0x77, 0x00, 0x65, 0x00, 0x72, 0x00, 0x50, 0x00,
                        0x6F, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x74, 0x00, 0x20, 0x00, 0x44, 0x00,
                        0x6F, 0x00, 0x63, 0x00, 0x75, 0x00, 0x6D, 0x00, 0x65, 0x00, 0x6E, 0x00,
                        0x74, 0x00,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x28, 0x52,
                            0x29, 0x20, 0x50, 0x6F, 0x77, 0x65, 0x72, 0x50, 0x6F, 0x69, 0x6E, 0x74,
                            0x20, 0x28, 0x52, 0x29, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73,
                            0x20, 0x20, 0x00, 0x07,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        Token::Literal(&[0x00, 0x00, 0xF0, 0x03, 0x00, 0x00, 0x5F]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 135,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 135,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 133,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
