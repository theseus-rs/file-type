use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1107: FileType = FileType {
    file_format: &FileFormat {
        id: 1_107,
        source_type: SourceType::Pronom,
        name: "pulse EKKO data file",
        extensions: &["dt1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x80, 0x3F]),
                        Token::WildcardCount(12),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(1),
                        Token::Range(&[0x41], &[0x42]),
                        Token::WildcardCount(8),
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(28),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(3),
                        Token::Range(&[0x40], &[0x49]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
