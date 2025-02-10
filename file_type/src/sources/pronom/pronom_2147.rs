use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2147: FileType = FileType {
    file_format: &FileFormat {
        id: 2_147,
        source_type: SourceType::Pronom,
        name: "Avery Label Pro Document",
        extensions: &["lpd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCC, 0xAA, 0x03, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
