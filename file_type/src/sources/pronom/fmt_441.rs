use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_441: FileFormat = FileFormat {
    id: 1_228,
    puid: "fmt/441",
    name: "Windows Media Video (WVC1)",
    extensions: &["wmv"],
    media_types: &[],
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
                            0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80,
                            0x5F, 0x5C, 0x44, 0x2B,
                        ]),
                        Token::WildcardCount(65),
                        Token::Literal(&[0x57, 0x56, 0x43, 0x31]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 691,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 692,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
