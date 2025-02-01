use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1850: FileFormat = FileFormat {
    id: 2_702,
    puid: "fmt/1850",
    name: "WordPerfect Macro File",
    extensions: &["wpm", "wcm"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x01, 0x01]),
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
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x01, 0x0A]),
                        Token::WildcardCountRange(0, 4_000),
                        Token::Any(&[
                            &[Token::Literal(&[0x77, 0x63, 0x6D])],
                            &[Token::Literal(&[0x57, 0x43, 0x4D])],
                        ]),
                        Token::AnyWildcard,
                        Token::Any(&[&[Token::Literal(&[0x61])], &[Token::Literal(&[0x41])]]),
                        Token::Literal(&[
                            0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        ]),
                        Token::WildcardCountRange(0, 15),
                        Token::Any(&[&[Token::Literal(&[0x64])], &[Token::Literal(&[0x44])]]),
                        Token::Literal(&[0x65, 0x66, 0x61, 0x75, 0x6C, 0x74]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 736,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 737,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
