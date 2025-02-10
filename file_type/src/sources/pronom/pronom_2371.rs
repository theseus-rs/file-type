use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2371: FileType = FileType {
    file_format: &FileFormat {
        id: 2_371,
        source_type: SourceType::Pronom,
        name: "Daisy-Dot Font File",
        extensions: &["nlq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x49, 0x53, 0x59, 0x2D, 0x44, 0x4F, 0x54, 0x20, 0x4E, 0x4C,
                        0x51, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
