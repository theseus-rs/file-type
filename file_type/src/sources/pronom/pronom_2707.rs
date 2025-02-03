use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2707: FileFormat = FileFormat {
    id: 2_707,
    source_type: SourceType::Pronom,
    name: "Open Media Framework Interchange",
    extensions: &["omf"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4F, 0x4D, 0x46, 0x49, 0x3A, 0x4F, 0x62, 0x6A, 0x49, 0x44,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xA4, 0x43, 0x4D, 0xA5, 0x48, 0x64, 0x72, 0xD7]),
                            Token::WildcardCount(4),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x00, 0x01])],
                            ]),
                        ],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4F, 0x4D, 0x46, 0x49, 0x3A, 0x4F, 0x62, 0x6A, 0x49, 0x44,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xA4, 0x43, 0x4D, 0xA5, 0x48, 0x64, 0x72, 0xD7]),
                            Token::WildcardCount(4),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x00, 0x01])],
                            ]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_709,
    }],
};
