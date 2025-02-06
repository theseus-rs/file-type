use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2209: FileFormat = FileFormat {
    id: 2_209,
    source_type: SourceType::Pronom,
    name: "Drawing Interchange Format (Binary)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x42, 0x69, 0x6E, 0x61,
                            0x72, 0x79, 0x20, 0x44, 0x58, 0x46, 0x0D, 0x0A, 0x1A, 0x00,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x00, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E, 0x00, 0x02,
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x09]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[
                            0x24, 0x41, 0x43, 0x41, 0x44, 0x56, 0x45, 0x52, 0x00, 0x01,
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x32, 0x34, 0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x00, 0x45, 0x4E, 0x44, 0x53, 0x45, 0x43, 0x00]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x45, 0x4F, 0x46, 0x00])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_212,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_210,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_208,
        },
    ],
};
