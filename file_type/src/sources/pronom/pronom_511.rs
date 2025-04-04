use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_511: FileType = FileType {
    file_format: &FileFormat {
        id: 511,
        source_type: SourceType::Pronom,
        name: "MultiMate Text File",
        extensions: &["dox", "fnx", "pat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(420),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x07]),
                        Token::WildcardCount(59),
                        Token::Literal(&[0x2E]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0x37]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x44, 0x24, 0x20, 0x20, 0x20, 0x20, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
