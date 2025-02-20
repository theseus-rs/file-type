use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2840: FileType = FileType {
    file_format: &FileFormat {
        id: 2_840,
        source_type: SourceType::Pronom,
        name: "Finale Performance Assessment",
        extensions: &["fpa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x49, 0x4E, 0x41, 0x4C, 0x45, 0x20, 0x50, 0x45, 0x52, 0x46, 0x4F,
                        0x52, 0x4D, 0x41, 0x4E, 0x43, 0x45, 0x20, 0x41, 0x53, 0x53, 0x45, 0x53,
                        0x53, 0x4D, 0x45, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
