use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2022: FileType = FileType {
    file_format: &FileFormat {
        id: 2_022,
        source_type: SourceType::Pronom,
        name: "HP System Software Manager CVA File",
        extensions: &["cva"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x56, 0x41, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x49, 0x6E,
                        0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
