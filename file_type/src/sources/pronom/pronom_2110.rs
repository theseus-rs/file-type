use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2110: FileType = FileType {
    file_format: &FileFormat {
        id: 2_110,
        source_type: SourceType::Pronom,
        name: "EIOffice Document",
        extensions: &["eio"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x45, 0x00,
                        0x56, 0x00, 0x45, 0x00, 0x52, 0x00, 0x4D, 0x00, 0x4F, 0x00, 0x52, 0x00,
                        0x45, 0x00, 0x20, 0x00, 0x53, 0x00, 0x4F, 0x00, 0x46, 0x00, 0x54, 0x00,
                        0x57, 0x00, 0x41, 0x00, 0x52, 0x00, 0x45, 0x00, 0x20, 0x00, 0x49, 0x00,
                        0x4E, 0x00, 0x43, 0x00, 0x2E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
