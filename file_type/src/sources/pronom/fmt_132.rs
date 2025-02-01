use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_132: FileFormat = FileFormat {
    id: 692,
    puid: "fmt/132",
    name: "Windows Media Audio",
    extensions: &["wma", "asf"],
    media_types: &["audio/x-ms-wma"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA,
                            0x00, 0x62, 0xCE, 0x6C,
                        ]),
                        Token::WildcardCount(12),
                        Token::Literal(&[0x01, 0x02]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::WildcardCount(30),
                        Token::Literal(&[
                            0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0x00, 0xC0,
                            0x0C, 0x20, 0x53, 0x65,
                        ]),
                        Token::WildcardCount(8),
                        Token::Literal(&[
                            0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80,
                            0x5F, 0x5C, 0x44, 0x2B,
                        ]),
                        Token::WildcardCount(38),
                        Token::Any(&[
                            &[Token::Literal(&[0x61, 0x01])],
                            &[Token::Literal(&[0x62, 0x01])],
                            &[Token::Literal(&[0x63, 0x01])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 693,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_228,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 691,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 691,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
