use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1976: FileType = FileType {
    file_format: &FileFormat {
        id: 1_976,
        source_type: SourceType::Pronom,
        name: "Niton Data Transfer",
        extensions: &["ndt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xBA, 0xFE, 0xD5, 0x46, 0x01, 0x00, 0x11, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
