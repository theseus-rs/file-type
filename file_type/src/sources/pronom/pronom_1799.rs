use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1799: FileType = FileType {
    file_format: &FileFormat {
        id: 1_799,
        source_type: SourceType::Pronom,
        name: "Jeffs Image Format",
        extensions: &["jif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x49, 0x46, 0x39, 0x39, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
