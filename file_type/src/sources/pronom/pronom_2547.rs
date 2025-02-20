use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2547: FileType = FileType {
    file_format: &FileFormat {
        id: 2_547,
        source_type: SourceType::Pronom,
        name: "Software602 Printer Configuration File",
        extensions: &["cfg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0x43, 0x36, 0x30, 0x32, 0x2D, 0x76, 0x2E, 0x31, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
