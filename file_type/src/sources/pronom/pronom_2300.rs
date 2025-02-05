use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2300: FileFormat = FileFormat {
    id: 2_300,
    source_type: SourceType::Pronom,
    name: "TEI P5 XML - Corpus File",
    extensions: &["xml", "tei", "odd"],
    media_types: &["application/tei+xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
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
                    Token::WildcardCountRange(0, 512),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x74, 0x65, 0x69, 0x43, 0x6F, 0x72, 0x70, 0x75, 0x73,
                        ])],
                        &[Token::Literal(&[
                            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x74,
                            0x65, 0x69, 0x2D, 0x63, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x6E, 0x73, 0x2F,
                            0x31, 0x2E, 0x30,
                        ])],
                    ]),
                    Token::WildcardCountRange(0, 512),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x74, 0x65, 0x69, 0x43, 0x6F, 0x72, 0x70, 0x75, 0x73,
                        ])],
                        &[Token::Literal(&[
                            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x74,
                            0x65, 0x69, 0x2D, 0x63, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x6E, 0x73, 0x2F,
                            0x31, 0x2E, 0x30,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_299,
        },
    ],
};
