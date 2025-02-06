use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1787: FileFormat = FileFormat {
    id: 1_787,
    source_type: SourceType::Pronom,
    name: "iMovieProj File Format",
    extensions: &["iMovieProj"],
    media_types: &[],
    signatures: &[Signature {
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
                        Token::WildcardCountRange(8, 256),
                        Token::Literal(&[
                            0x3C, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                            0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3E,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x4D, 0x6F, 0x76, 0x69, 0x65])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_784,
        },
    ],
};
