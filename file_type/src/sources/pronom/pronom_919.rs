use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_919: FileType = FileType {
    file_format: &FileFormat {
        id: 919,
        source_type: SourceType::Pronom,
        name: "FileMaker Pro Database",
        extensions: &["fp7"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(14),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xC0, 0x48, 0x42, 0x41, 0x4D, 0x37]),
                        Token::WildcardCount(505),
                        Token::Literal(&[
                            0x48, 0x42, 0x41, 0x4D, 0x32, 0x31, 0x30, 0x31, 0x4F, 0x43, 0x54, 0x39,
                            0x39, 0xC1, 0x02, 0x48, 0x07, 0x50, 0x72, 0x6F, 0x20, 0x37, 0x2E, 0x30,
                            0xC0, 0xC0,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
