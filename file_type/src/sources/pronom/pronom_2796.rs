use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2796: FileType = FileType {
    file_format: &FileFormat {
        id: 2_796,
        source_type: SourceType::Pronom,
        name: "S-57 Electronic Navigational Chart",
        extensions: &["000", "001", "002", "003", "004", "006"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(5),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x33, 0x4C, 0x45, 0x31, 0x20, 0x30, 0x39]),
                        Token::WildcardCountRange(0, 2_048),
                        Token::Literal(&[
                            0x49, 0x53, 0x4F, 0x2F, 0x49, 0x45, 0x43, 0x20, 0x38, 0x32, 0x31, 0x31,
                            0x20, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x20, 0x49, 0x64, 0x65, 0x6E,
                            0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
                        ]),
                        Token::WildcardCountRange(0, 128),
                        Token::Literal(&[
                            0x44, 0x61, 0x74, 0x61, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x64, 0x65,
                            0x6E, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                            0x66, 0x69, 0x65, 0x6C, 0x64,
                        ]),
                        Token::WildcardCountRange(0, 512),
                        Token::Literal(&[0x53, 0x54, 0x45, 0x44, 0x21]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Literal(&[0x1E]),
                        Token::WildcardCount(5),
                        Token::Literal(&[0x20, 0x44, 0x20, 0x20, 0x20, 0x20, 0x20]),
                        Token::WildcardCountRange(16, 256),
                        Token::Literal(&[0x30, 0x33, 0x2E, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
