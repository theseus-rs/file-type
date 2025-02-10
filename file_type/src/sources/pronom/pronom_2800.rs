use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2800: FileType = FileType {
    file_format: &FileFormat {
        id: 2_800,
        source_type: SourceType::Pronom,
        name: "Auto FX PhotoGraphic Edges Image File",
        extensions: &["afx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x88, 0x40, 0x47, 0x59, 0x0C, 0x0B, 0x1B, 0x0B, 0x00, 0x01, 0x01, 0x02,
                        0x01, 0x01, 0x05, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
