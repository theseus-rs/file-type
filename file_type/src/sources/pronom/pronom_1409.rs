use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1409: FileType = FileType {
    file_format: &FileFormat {
        id: 1_409,
        source_type: SourceType::Pronom,
        name: "RAR Archive",
        extensions: &["rar"],
        media_types: &["application/vnd.rar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
