use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2515: FileType = FileType {
    file_format: &FileFormat {
        id: 2_515,
        source_type: SourceType::Pronom,
        name: "Garmin track log file",
        extensions: &["gmn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x40, 0x67, 0x41, 0x72, 0x4D, 0x69, 0x4E, 0x40, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
