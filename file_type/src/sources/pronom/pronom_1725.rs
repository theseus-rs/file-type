use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1725: FileType = FileType {
    file_format: &FileFormat {
        id: 1_725,
        source_type: SourceType::Pronom,
        name: "AmiraMesh",
        extensions: &["am", "amiramesh", "hx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x41, 0x6D, 0x69, 0x72, 0x61, 0x4D, 0x65, 0x73, 0x68, 0x20,
                        0x33, 0x44, 0x20, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59, 0x2D, 0x4C, 0x49,
                        0x54, 0x54, 0x4C, 0x45, 0x2D, 0x45, 0x4E, 0x44, 0x49, 0x41, 0x4E, 0x20,
                        0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
