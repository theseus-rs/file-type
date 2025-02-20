use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1645: FileType = FileType {
    file_format: &FileFormat {
        id: 1_645,
        source_type: SourceType::Pronom,
        name: "Advanced Forensic Format",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x46, 0x46, 0x31, 0x30, 0x0D, 0x0A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
