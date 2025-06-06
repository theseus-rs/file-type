use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1724: FileType = FileType {
    file_format: &FileFormat {
        id: 1_724,
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
                        0x33, 0x44, 0x20, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59, 0x20, 0x32, 0x2E,
                        0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
