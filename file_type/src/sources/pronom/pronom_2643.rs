use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2643: FileType = FileType {
    file_format: &FileFormat {
        id: 2_643,
        source_type: SourceType::Pronom,
        name: "ICDRAW Group Icon File",
        extensions: &["ib3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
