use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1290: FileType = FileType {
    file_format: &FileFormat {
        id: 1_290,
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
                        0x00, 0x05, 0x16, 0x07, 0x00, 0x02, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
