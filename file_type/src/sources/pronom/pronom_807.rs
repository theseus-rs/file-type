use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_807: FileType = FileType {
    file_format: &FileFormat {
        id: 807,
        source_type: SourceType::Pronom,
        name: "Windows Setup File",
        extensions: &["inf"],
        media_types: &["application/inf"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Literal(&[0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Literal(&[0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3D]),
                            Token::WildcardCountRange(0, 2),
                            Token::Literal(&[0x24]),
                            Token::Any(&[
                                &[Token::Literal(&[0x43, 0x68, 0x69, 0x63, 0x61, 0x67, 0x6F])],
                                &[Token::Literal(&[0x43, 0x48, 0x49, 0x43, 0x41, 0x47, 0x4F])],
                            ]),
                            Token::Literal(&[0x24]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x0D, 0x0A]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Literal(&[0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Literal(&[0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65]),
                            Token::WildcardCountRange(0, 3),
                            Token::Literal(&[0x3D]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x22, 0x24]),
                            Token::Any(&[
                                &[Token::Literal(&[0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57, 0x53])],
                                &[Token::Literal(&[0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73])],
                            ]),
                            Token::Literal(&[0x20, 0x4E, 0x54, 0x24, 0x22, 0x0D, 0x0A]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Literal(&[0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Literal(&[0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65]),
                            Token::WildcardCountRange(0, 3),
                            Token::Literal(&[0x3D]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x22, 0x24]),
                            Token::Any(&[
                                &[Token::Literal(&[0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57, 0x53])],
                                &[Token::Literal(&[0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73])],
                            ]),
                            Token::Literal(&[0x20, 0x39, 0x35, 0x24, 0x22, 0x0D, 0x0A]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
