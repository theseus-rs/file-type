use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_63: FileType = FileType {
    file_format: &FileFormat {
        id: 63,
        source_type: SourceType::Pronom,
        name: "Harvard Graphics Chart",
        extensions: &["ch3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
