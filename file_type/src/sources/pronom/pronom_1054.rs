use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1054: FileType = FileType {
    file_format: &FileFormat {
        id: 1_054,
        source_type: SourceType::Pronom,
        name: "Open Financial Exchange",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x4F, 0x46, 0x58, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x3A, 0x31, 0x30,
                            0x30,
                        ]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x44, 0x41, 0x54, 0x41, 0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3A, 0x31, 0x30, 0x32,
                        ]),
                        Token::WildcardCountRange(0, 2),
                        Token::Literal(&[0x53, 0x45, 0x43, 0x55, 0x52, 0x49, 0x54, 0x59, 0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x45, 0x4E, 0x43, 0x4F, 0x44, 0x49, 0x4E, 0x47, 0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x43, 0x48, 0x41, 0x52, 0x53, 0x45, 0x54, 0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x43, 0x4F, 0x4D, 0x50, 0x52, 0x45, 0x53, 0x53, 0x49, 0x4F, 0x4E, 0x3A,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x4F, 0x4C, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x55, 0x49, 0x44, 0x3A,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x4E, 0x45, 0x57, 0x46, 0x49, 0x4C, 0x45, 0x55, 0x49, 0x44, 0x3A,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
