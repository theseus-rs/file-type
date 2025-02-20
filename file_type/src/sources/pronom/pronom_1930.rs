use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1930: FileType = FileType {
    file_format: &FileFormat {
        id: 1_930,
        source_type: SourceType::Pronom,
        name: "DIFFRACplus Raw Data File Format",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x41, 0x57, 0x31, 0x2E, 0x30, 0x30, 0x00]),
                        Token::WildcardCount(8),
                        Token::Range(&[0x30], &[0x31]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2F]),
                        Token::Range(&[0x30], &[0x33]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2F]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x39]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
