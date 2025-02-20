use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_18: FileType = FileType {
    file_format: &FileFormat {
        id: 18,
        source_type: SourceType::Pronom,
        name: "dBASE Database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x02]),
                            Token::WildcardCount(2),
                            Token::Range(&[0x01], &[0x1C]),
                            Token::Range(&[0x01], &[0x1F]),
                            Token::SingleWildcard,
                            Token::SingleWildcard,
                            Token::Range(&[0x00], &[0x03]),
                            Token::Any(&[
                                &[Token::Range(&[0x41], &[0x5A])],
                                &[Token::Range(&[0x61], &[0x7A])],
                            ]),
                            Token::WildcardCount(10),
                            Token::Any(&[
                                &[Token::Literal(&[0x43])],
                                &[Token::Literal(&[0x4E])],
                                &[Token::Literal(&[0x4C])],
                            ]),
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
                            Token::Literal(&[0x02]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::SingleWildcard,
                            Token::Range(&[0x00], &[0x03]),
                            Token::Any(&[
                                &[Token::Range(&[0x41], &[0x5A])],
                                &[Token::Range(&[0x61], &[0x7A])],
                            ]),
                            Token::WildcardCount(10),
                            Token::Any(&[
                                &[Token::Literal(&[0x43])],
                                &[Token::Literal(&[0x4E])],
                                &[Token::Literal(&[0x4C])],
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
