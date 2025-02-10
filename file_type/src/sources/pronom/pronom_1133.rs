use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1133: FileType = FileType {
    file_format: &FileFormat {
        id: 1_133,
        source_type: SourceType::Pronom,
        name: "Microsoft Windows Cursor",
        extensions: &["cur"],
        media_types: &["image/x-win-bitmap"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x00, 0x02, 0x00]),
                            Token::Range(&[0x01], &[0x09]),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x00]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x00, 0x00, 0x28, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                            Token::Range(&[0x01], &[0x20]),
                            Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x00, 0x28, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                            Token::Range(&[0x01], &[0x20]),
                            Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
