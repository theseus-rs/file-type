use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2275: FileType = FileType {
    file_format: &FileFormat {
        id: 2_275,
        source_type: SourceType::Pronom,
        name: "Lotus 1-2-3 Worksheet",
        extensions: &["123"],
        media_types: &["application/vnd.lotus-1-2-3", "application/x-123"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x1A, 0x00, 0x05, 0x10, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
