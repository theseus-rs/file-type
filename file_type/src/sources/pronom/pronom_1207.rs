use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1207: FileFormat = FileFormat {
    id: 1_207,
    source_type: SourceType::Pronom,
    name: "Video Object File (MPEG-2 subset)",
    extensions: &["vob"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x01, 0xBA]),
                        Token::WildcardCountRange(8, 11),
                        Token::Literal(&[0x00, 0x00, 0x01]),
                        Token::Any(&[
                            &[Token::Literal(&[0xBB])],
                            &[Token::Literal(&[0xBD])],
                            &[Token::Literal(&[0xBE])],
                            &[Token::Literal(&[0xE0])],
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x01, 0xBF, 0x03, 0xD4, 0x00]),
                        Token::WildcardCount(979),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xBF, 0x03, 0xFA, 0x01]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 659,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 660,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 660,
        },
    ],
};
