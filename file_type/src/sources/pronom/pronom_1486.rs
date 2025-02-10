use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1486: FileType = FileType {
    file_format: &FileFormat {
        id: 1_486,
        source_type: SourceType::Pronom,
        name: "Better Portable Graphics",
        extensions: &["bpg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x50, 0x47, 0xFB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
