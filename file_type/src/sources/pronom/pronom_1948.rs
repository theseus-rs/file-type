use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1948: FileType = FileType {
    file_format: &FileFormat {
        id: 1_948,
        source_type: SourceType::Pronom,
        name: "MiniCAD/VectorWorks",
        extensions: &["mcd", "vwx"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x09, 0x4E, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
