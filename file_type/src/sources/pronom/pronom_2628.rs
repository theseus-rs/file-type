use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2628: FileType = FileType {
    file_format: &FileFormat {
        id: 2_628,
        source_type: SourceType::Pronom,
        name: "Dynamic Publisher Picture File",
        extensions: &["pct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x59, 0x4E, 0x41, 0x4D, 0x49, 0x43, 0x20, 0x50, 0x55, 0x42, 0x4C,
                        0x49, 0x53, 0x48, 0x45, 0x52, 0x20, 0x53, 0x43, 0x52, 0x45, 0x45, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
