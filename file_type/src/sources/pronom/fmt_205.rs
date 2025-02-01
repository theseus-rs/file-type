use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_205: FileFormat = FileFormat {
    id: 931,
    puid: "fmt/205",
    name: "Synchronized Multimedia Integration Language (Generic)",
    extensions: &["smil", "smi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[0x31, 0x2E, 0x30]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::WildcardCountRange(20, 86),
                        Token::Literal(&[0x3C, 0x73, 0x6D, 0x69, 0x6C]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x2F, 0x73, 0x6D, 0x69, 0x6C, 0x3E])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
        RelatedFormat {
            id: 2_626,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
