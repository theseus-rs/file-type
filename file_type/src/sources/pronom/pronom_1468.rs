use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1468: FileType = FileType {
    file_format: &FileFormat {
        id: 1_468,
        source_type: SourceType::Pronom,
        name: "Minolta RAW",
        extensions: &["mrw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x4D, 0x52, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
