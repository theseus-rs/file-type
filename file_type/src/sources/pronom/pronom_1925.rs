use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1925: FileType = FileType {
    file_format: &FileFormat {
        id: 1_925,
        source_type: SourceType::Pronom,
        name: "Python Compiled File",
        extensions: &["pyc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x0D, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
