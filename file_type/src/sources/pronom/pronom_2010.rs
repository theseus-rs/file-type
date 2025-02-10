use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2010: FileType = FileType {
    file_format: &FileFormat {
        id: 2_010,
        source_type: SourceType::Pronom,
        name: "PowerDraw",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x64, 0x25, 0x34, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
