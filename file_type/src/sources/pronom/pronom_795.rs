use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_795: FileFormat = FileFormat {
    id: 795,
    source_type: SourceType::Pronom,
    name: "Digital Negative Format (DNG)",
    extensions: &["dng", "tif", "tiff"],
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
                            0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00,
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
                            0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00,
                        ])],
                    },
                },
            ],
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
                            0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x01, 0x00, 0x00,
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
                        tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x01, 0x00, 0x00,
                        ])],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 385,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 672,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 673,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 752,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 927,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_224,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_223,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 612,
        },
    ],
};
