use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2070: FileType = FileType {
    file_format: &FileFormat {
        id: 2_070,
        source_type: SourceType::Pronom,
        name: "Raw Flux Image",
        extensions: &["rfi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x46, 0x49, 0x7B, 0x64, 0x61, 0x74, 0x65, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
