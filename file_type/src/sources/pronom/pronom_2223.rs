use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2223: FileType = FileType {
    file_format: &FileFormat {
        id: 2_223,
        source_type: SourceType::Pronom,
        name: "Student Writing Center Letter",
        extensions: &["lt", "ltt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x1A, 0x54, 0x4C, 0x43]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x46, 0x46, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x04, 0x00])],
                                &[Token::Literal(&[0x00, 0x04])],
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x1A, 0x46, 0x46, 0x1A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
