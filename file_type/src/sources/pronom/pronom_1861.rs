use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1861: FileType = FileType {
    file_format: &FileFormat {
        id: 1_861,
        source_type: SourceType::Pronom,
        name: "SNAP Main Data File",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4E, 0x41, 0x50, 0x20, 0x4D, 0x61, 0x69, 0x6E, 0x20, 0x64, 0x61,
                        0x74, 0x61, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
