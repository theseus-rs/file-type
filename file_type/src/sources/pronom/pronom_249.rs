use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_249: FileType = FileType {
    file_format: &FileFormat {
        id: 249,
        source_type: SourceType::Pronom,
        name: "Picture Publisher Bitmap",
        extensions: &["pp4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x02, 0x01, 0x01, 0x00, 0x26, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
