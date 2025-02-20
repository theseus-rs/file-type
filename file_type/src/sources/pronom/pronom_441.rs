use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_441: FileType = FileType {
    file_format: &FileFormat {
        id: 441,
        source_type: SourceType::Pronom,
        name: "AMI Draw Vector Image",
        extensions: &["sdw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x49, 0x5F, 0x4D, 0x45, 0x54, 0x41, 0x46, 0x49, 0x4C, 0x45,
                        0x5F, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x20, 0x56, 0x45, 0x52, 0x53,
                        0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
