use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2125: FileType = FileType {
    file_format: &FileFormat {
        id: 2_125,
        source_type: SourceType::Pronom,
        name: "LocoScript Document",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4F, 0x59, 0x01, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
