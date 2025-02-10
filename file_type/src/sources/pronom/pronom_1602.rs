use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1602: FileType = FileType {
    file_format: &FileFormat {
        id: 1_602,
        source_type: SourceType::Pronom,
        name: "TAP (Commodore 64)",
        extensions: &["tap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x36, 0x34, 0x2D, 0x54, 0x41, 0x50, 0x45, 0x2D, 0x52, 0x41, 0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
