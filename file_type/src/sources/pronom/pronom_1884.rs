use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1884: FileType = FileType {
    file_format: &FileFormat {
        id: 1_884,
        source_type: SourceType::Pronom,
        name: "AVCHD Thumbnail Index File",
        extensions: &["tid"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x49, 0x44, 0x58, 0x30, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
