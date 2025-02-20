use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1109: FileType = FileType {
    file_format: &FileFormat {
        id: 1_109,
        source_type: SourceType::Pronom,
        name: "GSSI SIR-10 RADAN data file",
        extensions: &["dzt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[0xFF, 0x00])],
                            &[Token::Literal(&[0x00, 0x07])],
                        ]),
                        Token::Literal(&[0x00, 0x04, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x04])]]),
                        Token::Literal(&[0x10, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x80])]]),
                        Token::Literal(&[0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x80])]]),
                        Token::Range(&[0x41], &[0x42]),
                        Token::WildcardCount(84),
                        Token::Range(&[0x31], &[0x39]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::WildcardCount(13),
                        Token::Any(&[
                            &[Token::Literal(&[0x46, 0x49, 0x4C, 0x45])],
                            &[Token::Literal(&[0x43, 0x4C, 0x41, 0x53])],
                        ]),
                        Token::WildcardCount(906),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x00])],
                            &[Token::Literal(&[0xFF, 0xFF])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
