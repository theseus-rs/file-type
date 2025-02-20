use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2308: FileType = FileType {
    file_format: &FileFormat {
        id: 2_308,
        source_type: SourceType::Pronom,
        name: "JPEG XL",
        extensions: &["jxl"],
        media_types: &["image/jxl"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
