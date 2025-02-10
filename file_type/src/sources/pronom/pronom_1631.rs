use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1631: FileType = FileType {
    file_format: &FileFormat {
        id: 1_631,
        source_type: SourceType::Pronom,
        name: "Qsplat Model",
        extensions: &["qs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x53, 0x70, 0x6C, 0x61, 0x74, 0x31, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
