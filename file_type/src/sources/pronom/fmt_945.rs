use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_945: FileFormat = FileFormat {
    id: 1_750,
    puid: "fmt/945",
    name: "Ogg Theora Video",
    extensions: &["ogv", "ogg"],
    media_types: &["video/ogg"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                        Token::WildcardCount(22),
                        Token::Literal(&[
                            0x80, 0x74, 0x68, 0x65, 0x6F, 0x72, 0x61, 0x03, 0x02, 0x01,
                        ]),
                        Token::WildcardCount(32),
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53]),
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
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                        Token::WildcardCount(22),
                        Token::Literal(&[0x66, 0x69, 0x73, 0x68, 0x65, 0x61, 0x64, 0x00]),
                        Token::WildcardCountRange(56, 72),
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                        Token::WildcardCount(22),
                        Token::Literal(&[
                            0x80, 0x74, 0x68, 0x65, 0x6F, 0x72, 0x61, 0x03, 0x02, 0x01,
                        ]),
                        Token::WildcardCount(32),
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_749,
            relationship_type: RelationshipType::CanBeContainedBy,
        },
        RelatedFormat {
            id: 1_749,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
