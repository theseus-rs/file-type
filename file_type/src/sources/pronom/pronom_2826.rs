use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2826: FileType = FileType {
    file_format: &FileFormat {
        id: 2_826,
        source_type: SourceType::Pronom,
        name: "Shorten (codec)",
        extensions: &["shn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x6A, 0x6B, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
