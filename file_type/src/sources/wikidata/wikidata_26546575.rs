use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26546575: FileFormat = FileFormat {
    id: 26_546_575,
    source_type: SourceType::Wikidata,
    name: "Portable Document Format/Archive, version 2 Basic",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64,
                            0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[
                            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x61,
                            0x69, 0x69, 0x6D, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x70, 0x64, 0x66, 0x61,
                            0x2F, 0x6E, 0x73, 0x2F, 0x69, 0x64,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61, 0x72, 0x74,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x3D, 0x22])],
                            &[Token::Literal(&[0x3D, 0x27])],
                            &[Token::Literal(&[0x3E])],
                        ]),
                        Token::Literal(&[0x32]),
                        Token::Any(&[
                            &[Token::Literal(&[0x22])],
                            &[Token::Literal(&[0x27])],
                            &[Token::Literal(&[
                                0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61,
                                0x72, 0x74, 0x3E,
                            ])],
                        ]),
                        Token::WildcardCountRange(0, 11),
                        Token::Literal(&[
                            0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63, 0x6F, 0x6E, 0x66, 0x6F,
                            0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x3E])],
                            &[Token::Literal(&[0x3D, 0x22])],
                            &[Token::Literal(&[0x3D, 0x27])],
                        ]),
                        Token::Literal(&[0x42]),
                        Token::Any(&[
                            &[Token::Literal(&[0x22])],
                            &[Token::Literal(&[0x27])],
                            &[Token::Literal(&[
                                0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x63, 0x6F,
                                0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x6E, 0x63, 0x65, 0x3E,
                            ])],
                        ]),
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
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E]),
                        Token::Range(&[0x30], &[0x37]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
