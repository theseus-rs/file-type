use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1768: FileType = FileType {
    file_format: &FileFormat {
        id: 1_768,
        source_type: SourceType::Pronom,
        name: "OMNIC Spectral Data File",
        extensions: &["spa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x61, 0x6C, 0x20, 0x44, 0x61, 0x74,
                        0x61, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
