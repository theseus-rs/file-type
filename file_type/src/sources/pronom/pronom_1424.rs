use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1424: FileType = FileType {
    file_format: &FileFormat {
        id: 1_424,
        source_type: SourceType::Pronom,
        name: "Apple Disk Copy Image",
        extensions: &["dmg", "smi", "img", "image"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(64),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x06, 0x40, 0x00])],
                            &[Token::Literal(&[0x00, 0x0C, 0x80, 0x00])],
                            &[Token::Literal(&[0x00, 0x0B, 0x40, 0x00])],
                            &[Token::Literal(&[0x00, 0x16, 0x80, 0x00])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x00, 0x25, 0x80])],
                            &[Token::Literal(&[0x00, 0x00, 0x4B, 0x00])],
                            &[Token::Literal(&[0x00, 0x00, 0x00, 0x00])],
                        ]),
                        Token::WildcardCount(8),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x12])],
                            &[Token::Literal(&[0x22])],
                            &[Token::Literal(&[0x24])],
                            &[Token::Literal(&[0x96])],
                        ]),
                        Token::Literal(&[0x01, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
