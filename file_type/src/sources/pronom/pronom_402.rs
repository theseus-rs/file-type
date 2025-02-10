use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_402: FileType = FileType {
    file_format: &FileFormat {
        id: 402,
        source_type: SourceType::Pronom,
        name: "OS/2 Bitmap",
        extensions: &["bmp"],
        media_types: &["image/bmp"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x42, 0x4D]),
                            Token::WildcardCount(12),
                            Token::Any(&[
                                &[Token::Literal(&[0x10])],
                                &[Token::Literal(&[0x30])],
                                &[Token::Literal(&[0x40])],
                            ]),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::WildcardCount(8),
                            Token::Literal(&[0x01, 0x00]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x42, 0x4D]),
                            Token::WildcardCount(12),
                            Token::Literal(&[0x28, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(8),
                            Token::Literal(&[0x01, 0x00, 0x18, 0x00, 0x04, 0x00, 0x00, 0x00]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x42, 0x4D]),
                            Token::WildcardCount(12),
                            Token::Literal(&[0x28, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(8),
                            Token::Literal(&[0x01, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
