use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2750: FileType = FileType {
    file_format: &FileFormat {
        id: 2_750,
        source_type: SourceType::Pronom,
        name: "RagTime Document File",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(16),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x68, 0x8F, 0x68, 0x8F, 0x68, 0x8F, 0xFF, 0xF3,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
