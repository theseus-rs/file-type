use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2412: FileType = FileType {
    file_format: &FileFormat {
        id: 2_412,
        source_type: SourceType::Pronom,
        name: "COKE Format (Atari Falcon)",
        extensions: &["tg1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4B, 0x45, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
