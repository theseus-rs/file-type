use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2621: FileFormat = FileFormat {
    id: 2_621,
    source_type: SourceType::Pronom,
    name: "ESRI Persistent Auxiliary Metadata File",
    extensions: &["xml", "aux.xml"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x50, 0x41, 0x4D, 0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x3E,
                        ]),
                        Token::WildcardCountRange(10, 200),
                        Token::Literal(&[
                            0x3C, 0x4D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x64, 0x6F,
                            0x6D, 0x61, 0x69, 0x6E, 0x3D, 0x22,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x45, 0x53, 0x52, 0x49, 0x22])],
                            &[Token::Literal(&[0x45, 0x73, 0x72, 0x69, 0x22])],
                        ]),
                        Token::Literal(&[0x3E]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x2F, 0x50, 0x41, 0x4D, 0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74,
                        0x3E,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
