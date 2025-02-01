use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_280: FileFormat = FileFormat {
    id: 429,
    puid: "x-fmt/280",
    name: "XML Schema Definition",
    extensions: &["xsd"],
    media_types: &["application/xml"],
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
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C]),
                        Token::Any(&[
                            &[Token::Literal(&[0x2F, 0x78, 0x73, 0x64, 0x3A])],
                            &[Token::Literal(&[0x2F, 0x78, 0x73, 0x3A])],
                            &[Token::Literal(&[0x2F])],
                        ]),
                        Token::Literal(&[0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x3E]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 319,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_852,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_192,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
