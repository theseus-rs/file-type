use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_130: FileType = FileType {
    file_format: &FileFormat {
        id: 130,
        source_type: SourceType::Pronom,
        name: "Picture Publisher Bitmap",
        extensions: &["pp5"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x55, 0x42, 0x49, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
