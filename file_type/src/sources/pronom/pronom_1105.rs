use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1105: FileType = FileType {
    file_format: &FileFormat {
        id: 1_105,
        source_type: SourceType::Pronom,
        name: "Microsoft Front Page Binary Tree Index",
        extensions: &["btr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x1C, 0x01, 0x00, 0x00]),
                        Token::WildcardCount(272),
                        Token::Literal(&[0x0C, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
