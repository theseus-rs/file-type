use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1521: FileType = FileType {
    file_format: &FileFormat {
        id: 1_521,
        source_type: SourceType::Pronom,
        name: "Oktalyzer Audio file",
        extensions: &["okt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x4B, 0x54, 0x41, 0x53, 0x4F, 0x4E, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
