use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_141: FileType = FileType {
    file_format: &FileFormat {
        id: 141,
        source_type: SourceType::Pronom,
        name: "Pocket Word Document",
        extensions: &["psw", "pwd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x5C, 0x70, 0x77, 0x64, 0x32, 0x5C, 0x61, 0x6E, 0x73, 0x69,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
