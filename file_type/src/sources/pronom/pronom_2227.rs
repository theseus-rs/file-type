use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2227: FileType = FileType {
    file_format: &FileFormat {
        id: 2_227,
        source_type: SourceType::Pronom,
        name: "Flow Charting",
        extensions: &["fc5"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0xFF, 0xCD, 0xAB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
