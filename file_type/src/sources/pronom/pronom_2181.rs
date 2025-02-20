use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2181: FileType = FileType {
    file_format: &FileFormat {
        id: 2_181,
        source_type: SourceType::Pronom,
        name: "DeluxePaint Animation File",
        extensions: &["anm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x50, 0x46, 0x20, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
