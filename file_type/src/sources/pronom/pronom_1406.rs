use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1406: FileType = FileType {
    file_format: &FileFormat {
        id: 1_406,
        source_type: SourceType::Pronom,
        name: "ARJ File Format",
        extensions: &["arj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x60, 0xEA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
