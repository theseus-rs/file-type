use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1081: FileType = FileType {
    file_format: &FileFormat {
        id: 1_081,
        source_type: SourceType::Pronom,
        name: "Graphic Workshop for Windows Thumbnail File",
        extensions: &["thn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x48, 0x4E, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
