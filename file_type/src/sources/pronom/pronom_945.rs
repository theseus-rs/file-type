use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_945: FileType = FileType {
    file_format: &FileFormat {
        id: 945,
        source_type: SourceType::Pronom,
        name: "PaintShop Pro Browser Cache File",
        extensions: &["jbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x41, 0x53, 0x43, 0x20, 0x42, 0x52, 0x4F, 0x57, 0x53, 0x20, 0x46,
                        0x49, 0x4C, 0x45, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
