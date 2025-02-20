use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1911: FileType = FileType {
    file_format: &FileFormat {
        id: 1_911,
        source_type: SourceType::Pronom,
        name: "AutoCAD Hatch Pattern",
        extensions: &["pat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x2A]),
                        Token::Any(&[
                            &[Token::Range(&[0x30], &[0x39])],
                            &[Token::Range(&[0x41], &[0x5A])],
                            &[Token::Range(&[0x61], &[0x7A])],
                        ]),
                        Token::WildcardCountRange(0, 80),
                        Token::Literal(&[0x0D, 0x0A]),
                        Token::WildcardCountRange(0, 10),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2C, 0x20]),
                        Token::WildcardCountRange(0, 10),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2C]),
                        Token::Any(&[
                            &[Token::Range(&[0x30], &[0x39])],
                            &[Token::Literal(&[0x2D]), Token::Range(&[0x30], &[0x39])],
                            &[Token::Literal(&[0x2E]), Token::Range(&[0x30], &[0x39])],
                        ]),
                        Token::WildcardCountRange(0, 10),
                        Token::Literal(&[0x2C, 0x20]),
                        Token::WildcardCountRange(0, 10),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2C]),
                        Token::Any(&[
                            &[Token::Range(&[0x30], &[0x39])],
                            &[Token::Literal(&[0x2D]), Token::Range(&[0x30], &[0x39])],
                            &[Token::Literal(&[0x2E]), Token::Range(&[0x30], &[0x39])],
                        ]),
                        Token::WildcardCountRange(0, 128),
                        Token::Literal(&[0x0D, 0x0A]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
