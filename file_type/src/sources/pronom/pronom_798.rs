use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_798: FileFormat = FileFormat {
    id: 798,
    source_type: SourceType::Pronom,
    name: "Geographic Tagged Image File Format (GeoTIFF)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCountRange(4, 4_080),
                        Token::Literal(&[0xAF, 0x87, 0x03, 0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00, 0x00, 0x00]),
                    ],
                },
            }],
        },
        Signature {
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
                    offset: Some(4),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xAF, 0x87, 0x03, 0x00]),
                            Token::SingleWildcard,
                            Token::Literal(&[0x00, 0x00, 0x00]),
                        ],
                    },
                },
            ],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                        Token::WildcardCountRange(4, 4_080),
                        Token::Literal(&[0x87, 0xAF, 0x00, 0x03, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        },
        Signature {
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
                    offset: Some(5),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x87, 0xAF, 0x00, 0x03, 0x00, 0x00, 0x00])],
                    },
                },
            ],
        },
    ],
    related_formats: &[
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
            id: 1_099,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 1_099,
        },
    ],
};
