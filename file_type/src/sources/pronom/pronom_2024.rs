use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2024: FileType = FileType {
    file_format: &FileFormat {
        id: 2_024,
        source_type: SourceType::Pronom,
        name: "Cakewalk WRK Project",
        extensions: &["wrk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x4B, 0x45, 0x57, 0x41, 0x4C, 0x4B, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
