use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1851: FileType = FileType {
    file_format: &FileFormat {
        id: 1_851,
        source_type: SourceType::Pronom,
        name: "Draco File Format",
        extensions: &["drc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x52, 0x41, 0x43, 0x4F, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
