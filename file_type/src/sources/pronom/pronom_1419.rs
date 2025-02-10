use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1419: FileType = FileType {
    file_format: &FileFormat {
        id: 1_419,
        source_type: SourceType::Pronom,
        name: "SmartDraw",
        extensions: &["sdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4D, 0x41, 0x52, 0x54, 0x44, 0x52, 0x57, 0x01, 0x80,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
