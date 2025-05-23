use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1605: FileType = FileType {
    file_format: &FileFormat {
        id: 1_605,
        source_type: SourceType::Pronom,
        name: "XAML Binary Format",
        extensions: &["xbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x42, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
