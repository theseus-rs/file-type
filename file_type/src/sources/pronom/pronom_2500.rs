use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2500: FileType = FileType {
    file_format: &FileFormat {
        id: 2_500,
        source_type: SourceType::Pronom,
        name: "RED Thumbnail File",
        extensions: &["rtn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x52, 0x45, 0x44, 0x54, 0x48, 0x55, 0x4D, 0x42, 0x4E, 0x41, 0x49, 0x4C,
                            0x03, 0x18,
                        ]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x72, 0x64, 0x69, 0x01]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x18]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x0C, 0x58, 0x11, 0x02, 0x03, 0xE8]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x61, 0xA8]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
