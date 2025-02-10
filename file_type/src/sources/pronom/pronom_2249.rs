use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2249: FileType = FileType {
    file_format: &FileFormat {
        id: 2_249,
        source_type: SourceType::Pronom,
        name: "Minitab Portable Worksheet",
        extensions: &["mtp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x6E, 0x69, 0x74, 0x61, 0x62, 0x20, 0x50, 0x6F, 0x72, 0x74,
                        0x61, 0x62, 0x6C, 0x65, 0x20, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x68, 0x65,
                        0x65, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
