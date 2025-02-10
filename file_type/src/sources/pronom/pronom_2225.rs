use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2225: FileType = FileType {
    file_format: &FileFormat {
        id: 2_225,
        source_type: SourceType::Pronom,
        name: "Flow Charting",
        extensions: &["fcd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x4F, 0x43, 0x48, 0x54, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
