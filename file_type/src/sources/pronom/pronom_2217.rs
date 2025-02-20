use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2217: FileType = FileType {
    file_format: &FileFormat {
        id: 2_217,
        source_type: SourceType::Pronom,
        name: "DiskDoubler",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAB, 0xCD, 0x00, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
