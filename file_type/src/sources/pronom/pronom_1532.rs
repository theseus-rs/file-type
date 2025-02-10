use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1532: FileType = FileType {
    file_format: &FileFormat {
        id: 1_532,
        source_type: SourceType::Pronom,
        name: "FL Studio project file (FLP)",
        extensions: &["flp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x68, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
