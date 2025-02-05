use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1899: FileFormat = FileFormat {
    id: 1_899,
    source_type: SourceType::Pronom,
    name: "Scribus Document",
    extensions: &["sla", "scd"],
    media_types: &["application/vnd.scribus"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x53, 0x43, 0x52, 0x49, 0x42, 0x55, 0x53,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                            0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                            0x3F, 0x3E,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D, 0x0A])],
                        ]),
                        Token::Literal(&[
                            0x3C, 0x53, 0x43, 0x52, 0x49, 0x42, 0x55, 0x53, 0x55, 0x54, 0x46, 0x38,
                            0x4E, 0x45, 0x57,
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
