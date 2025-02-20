use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1736: FileType = FileType {
    file_format: &FileFormat {
        id: 1_736,
        source_type: SourceType::Pronom,
        name: "Mathcad Document",
        extensions: &["mcd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x4D, 0x43, 0x41, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
