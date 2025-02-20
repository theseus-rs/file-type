use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1063: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063,
        source_type: SourceType::Pronom,
        name: "Secure DjVU",
        extensions: &["djvu", "djv"],
        media_types: &["image/vnd.djvu", "image/x-djvu"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x44, 0x4A, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
