use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2727: FileType = FileType {
    file_format: &FileFormat {
        id: 2_727,
        source_type: SourceType::Pronom,
        name: "Guitar Pro File",
        extensions: &["gp3", "gp4", "gp5"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x18, 0x46, 0x49, 0x43, 0x48, 0x49, 0x45, 0x52, 0x20, 0x47, 0x55, 0x49,
                        0x54, 0x41, 0x52, 0x20, 0x50, 0x52, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
