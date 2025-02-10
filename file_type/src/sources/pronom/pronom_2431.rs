use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2431: FileType = FileType {
    file_format: &FileFormat {
        id: 2_431,
        source_type: SourceType::Pronom,
        name: "EggPaint (Atari Falcon)",
        extensions: &["trp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x52, 0x55, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
