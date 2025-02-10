use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2094: FileType = FileType {
    file_format: &FileFormat {
        id: 2_094,
        source_type: SourceType::Pronom,
        name: "SureThing Project File",
        extensions: &["std"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x56, 0x00, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
