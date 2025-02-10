use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1891: FileType = FileType {
    file_format: &FileFormat {
        id: 1_891,
        source_type: SourceType::Pronom,
        name: "Hangul Word Processor Document",
        extensions: &["hwp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x57, 0x50, 0x20, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                        0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x33, 0x2E, 0x30, 0x30, 0x20,
                        0x1A, 0x01, 0x02, 0x03, 0x04, 0x05,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
