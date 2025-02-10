use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2510: FileType = FileType {
    file_format: &FileFormat {
        id: 2_510,
        source_type: SourceType::Pronom,
        name: "ZyXEL Voice Format Audio",
        extensions: &["zvd", "zyx", "ad2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
