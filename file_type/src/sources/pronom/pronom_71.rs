use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_71: FileType = FileType {
    file_format: &FileFormat {
        id: 71,
        source_type: SourceType::Pronom,
        name: "AutoCAD dbConnect Template Set",
        extensions: &["dbt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x00, 0x00, 0x00, 0x44, 0x42, 0x43, 0x4E, 0x43, 0x20, 0x52, 0x31,
                        0x35, 0x7C, 0x0C, 0x00, 0x00, 0x00, 0x54, 0x4D, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
