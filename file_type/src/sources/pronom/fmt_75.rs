use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_75: FileFormat = FileFormat {
    id: 721,
    puid: "fmt/75",
    name: "Drawing Interchange File Format (ASCII)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x30]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0A])],
                        ]),
                        Token::Literal(&[0x45, 0x4F, 0x46]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x30]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x20, 0x20, 0x32]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x45, 0x52]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x39]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x24, 0x41, 0x43, 0x41, 0x44, 0x56, 0x45, 0x52]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x20, 0x20, 0x31]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x30, 0x39]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x30]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x45, 0x4E, 0x44, 0x53, 0x45, 0x43]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 740,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 705,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 766,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 722,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 720,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
