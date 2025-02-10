use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2739: FileType = FileType {
    file_format: &FileFormat {
        id: 2_739,
        source_type: SourceType::Pronom,
        name: "CloudCompare Entity File",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x43, 0x42, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
