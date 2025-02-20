use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1487: FileType = FileType {
    file_format: &FileFormat {
        id: 1_487,
        source_type: SourceType::Pronom,
        name: "Executable and Linkable Format",
        extensions: &["elf", "o"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
