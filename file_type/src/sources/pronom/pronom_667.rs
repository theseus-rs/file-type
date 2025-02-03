use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_667: FileFormat = FileFormat {
    id: 667,
    source_type: SourceType::Pronom,
    name: "JPEG File Interchange Format",
    extensions: &["jpeg", "jpe", "jpg", "jif", "jfif", "jfi"],
    media_types: &["image/jpeg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0xD8, 0xFF, 0xE0]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xD9])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 670,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 668,
        },
    ],
};
