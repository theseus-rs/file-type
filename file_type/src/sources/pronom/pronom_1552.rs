use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1552: FileType = FileType {
    file_format: &FileFormat {
        id: 1_552,
        source_type: SourceType::Pronom,
        name: "AppleWorks Presentation",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x06]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                        Token::WildcardCount(270),
                        Token::Literal(&[0x05]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
