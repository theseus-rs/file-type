use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2031: FileType = FileType {
    file_format: &FileFormat {
        id: 2_031,
        source_type: SourceType::Pronom,
        name: "WordPerfect for Macintosh Document",
        extensions: &[],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x01, 0x2C, 0x03]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
