use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_143: FileType = FileType {
    file_format: &FileFormat {
        id: 143,
        source_type: SourceType::Pronom,
        name: "Inkwriter/Notetaker Document",
        extensions: &["pwi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
                },
            }],
        }],
        related_formats: &[],
    },
};
