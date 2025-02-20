use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1894: FileType = FileType {
    file_format: &FileFormat {
        id: 1_894,
        source_type: SourceType::Pronom,
        name: "Monkey's Audio File",
        extensions: &["ape"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D, 0x41, 0x43, 0x20]),
                            Token::WildcardCount(48),
                            Token::Any(&[
                                &[Token::Literal(&[0xE8, 0x03])],
                                &[Token::Literal(&[0xD0, 0x07])],
                                &[Token::Literal(&[0xB8, 0x0B])],
                                &[Token::Literal(&[0xA0, 0x0F])],
                                &[Token::Literal(&[0x88, 0x13])],
                            ]),
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
                            Token::Literal(&[0x4D, 0x41, 0x43, 0x20]),
                            Token::WildcardCount(2),
                            Token::Any(&[
                                &[Token::Literal(&[0xE8, 0x03])],
                                &[Token::Literal(&[0xD0, 0x07])],
                                &[Token::Literal(&[0xB8, 0x0B])],
                                &[Token::Literal(&[0xA0, 0x0F])],
                                &[Token::Literal(&[0x88, 0x13])],
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
