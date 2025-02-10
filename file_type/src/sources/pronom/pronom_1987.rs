use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1987: FileType = FileType {
    file_format: &FileFormat {
        id: 1_987,
        source_type: SourceType::Pronom,
        name: "MicroStation Material Library",
        extensions: &["mat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x56, 0x5F, 0x41, 0x53, 0x2D, 0x2D, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
