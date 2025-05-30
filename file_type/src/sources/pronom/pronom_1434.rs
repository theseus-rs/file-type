use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1434: FileType = FileType {
    file_format: &FileFormat {
        id: 1_434,
        source_type: SourceType::Pronom,
        name: "CPIO",
        extensions: &["cpio"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC7, 0x71])],
                },
            }],
        }],
        related_formats: &[],
    },
};
