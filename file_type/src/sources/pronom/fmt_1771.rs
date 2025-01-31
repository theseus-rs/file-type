use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1771: FileFormat = FileFormat {
    id: 2_621,
    puid: "fmt/1771",
    name: "ESRI Persistent Auxiliary Metadata File",
    extensions: &["xml", "aux.xml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
