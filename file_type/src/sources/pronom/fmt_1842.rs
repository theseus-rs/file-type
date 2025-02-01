use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1842: FileFormat = FileFormat {
    id: 2_694,
    puid: "fmt/1842",
    name: "Digital Negative Format (DNG)",
    extensions: &["dng"],
    media_types: &["image/dng", "image/tiff"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCountRange(0, 4_080),
                        Token::Literal(&[
                            0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x06, 0x00, 0x00,
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                        Token::WildcardCountRange(0, 4_080),
                        Token::Literal(&[
                            0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x06, 0x00, 0x00,
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x06, 0x00, 0x00,
                        ])],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x06, 0x00, 0x00,
                        ])],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 672,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 673,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 752,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 797,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 927,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_099,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_804,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_693,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 1_099,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
