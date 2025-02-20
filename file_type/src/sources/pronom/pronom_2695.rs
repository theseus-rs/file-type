use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2695: FileType = FileType {
    file_format: &FileFormat {
        id: 2_695,
        source_type: SourceType::Pronom,
        name: "Human Machine Interfaces HMI File",
        extensions: &["hmi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x4D, 0x49, 0x2D, 0x4D, 0x49, 0x44, 0x49, 0x53, 0x4F, 0x4E, 0x47,
                        0x30, 0x36, 0x31, 0x35, 0x39, 0x35,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
