use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_155: FileFormat = FileFormat {
    id: 798,
    puid: "fmt/155",
    name: "Geographic Tagged Image File Format (GeoTIFF)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    internal_signatures: &[
        InternalSignature {
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
        InternalSignature {
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
            id: 1_099,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_099,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
