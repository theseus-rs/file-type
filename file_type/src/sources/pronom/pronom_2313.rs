use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2313: FileType = FileType {
    file_format: &FileFormat {
        id: 2_313,
        source_type: SourceType::Pronom,
        name: "HyperCard Stack",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x54, 0x41, 0x4B, 0xFF, 0xFF, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
