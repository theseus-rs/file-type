use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1581: FileType = FileType {
    file_format: &FileFormat {
        id: 1_581,
        source_type: SourceType::Pronom,
        name: "PowerVR Object Data",
        extensions: &["pod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xE8, 0x03, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x41, 0x42, 0x2E, 0x50,
                        0x4F, 0x44, 0x2E, 0x32, 0x2E, 0x30, 0x00, 0xE8, 0x03, 0x00, 0x80, 0x00,
                        0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
