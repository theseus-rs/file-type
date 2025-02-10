use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1375: FileType = FileType {
    file_format: &FileFormat {
        id: 1_375,
        source_type: SourceType::Pronom,
        name: "LifeTechnologies ABIF",
        extensions: &["abif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42, 0x49, 0x46, 0x00, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
