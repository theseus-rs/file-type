use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1750: FileFormat = FileFormat {
    id: 1_750,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::CanBeContainedBy,
            id: 1_749,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_749,
        },
    ],
};
