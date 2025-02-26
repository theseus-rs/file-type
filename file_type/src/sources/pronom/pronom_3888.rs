use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3888: FileType = FileType {
    file_format: &FileFormat {
        id: 3_888,
        source_type: SourceType::Pronom,
        name: "Axon Binary Format",
        extensions: &["abf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
