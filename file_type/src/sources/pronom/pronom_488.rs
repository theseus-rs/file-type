use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_488: FileType = FileType {
    file_format: &FileFormat {
        id: 488,
        source_type: SourceType::Pronom,
        name: "Harvard Graphics Vector Graphics",
        extensions: &["cht"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x61, 0x6C, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
