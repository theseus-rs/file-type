use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_533: FileFormat = FileFormat {
    id: 533,
    source_type: SourceType::Pronom,
    name: "Truevision TGA Bitmap",
    extensions: &["tga", "icb", "vda", "vst", "afi", "bpx"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x01]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x09])]]),
                        Token::WildcardCount(4),
                        Token::Any(&[
                            &[Token::Literal(&[0x0F])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::WildcardCount(8),
                        Token::Any(&[
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x0F])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0B])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(8),
                        Token::Any(&[
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x0F])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_150,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_672,
        },
    ],
};
