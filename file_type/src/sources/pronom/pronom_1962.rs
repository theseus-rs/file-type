use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1962: FileType = FileType {
    file_format: &FileFormat {
        id: 1_962,
        source_type: SourceType::Pronom,
        name: "Lightwright Show File",
        extensions: &["lw2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x57, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x20, 0x56, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
