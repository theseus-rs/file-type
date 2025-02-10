use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const CUSTOM_1: FileType = FileType {
    file_format: &FileFormat {
        id: 1,
        source_type: SourceType::Custom,
        name: "Apache Arrow",
        extensions: &["arrow"],
        media_types: &["application/vnd.apache.arrow.file"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x52, 0x52, 0x4F, 0x57, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
