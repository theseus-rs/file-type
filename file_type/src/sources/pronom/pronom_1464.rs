use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1464: FileType = FileType {
    file_format: &FileFormat {
        id: 1_464,
        source_type: SourceType::Pronom,
        name: "Chasys Draw image file",
        extensions: &["cd5"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x43, 0x44, 0x35, 0x10, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x03,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
