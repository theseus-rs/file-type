use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1771: FileType = FileType {
    file_format: &FileFormat {
        id: 1_771,
        source_type: SourceType::Pronom,
        name: "AppleDouble Resource Fork",
        extensions: &[],
        media_types: &["multipart/appledouble"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x05, 0x16, 0x07, 0x00, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
