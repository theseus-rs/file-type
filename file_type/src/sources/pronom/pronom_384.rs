use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_384: FileType = FileType {
    file_format: &FileFormat {
        id: 384,
        source_type: SourceType::Pronom,
        name: "RAR Archive",
        extensions: &["rar"],
        media_types: &["application/vnd.rar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x73]),
                        Token::WildcardCount(34),
                        Token::Literal(&[0x14]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
