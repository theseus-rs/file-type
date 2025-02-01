use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1613: FileFormat = FileFormat {
    id: 2_440,
    puid: "fmt/1613",
    name: "XML Shareable Playlist Format",
    extensions: &["xspf"],
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
                        Token::WildcardCountRange(25, 26),
                        Token::Literal(&[0x3C, 0x70, 0x6C, 0x61, 0x79, 0x6C, 0x69, 0x73, 0x74]),
                        Token::WildcardCountRange(1, 14),
                        Token::Literal(&[
                            0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                            0x2F, 0x2F, 0x78, 0x73, 0x70, 0x66, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x6E,
                            0x73, 0x2F, 0x30, 0x2F,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x2F, 0x70, 0x6C, 0x61, 0x79, 0x6C, 0x69, 0x73, 0x74, 0x3E,
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
