use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1149: FileType = FileType {
    file_format: &FileFormat {
        id: 1_149,
        source_type: SourceType::Pronom,
        name: "X-Windows Screen Dump",
        extensions: &["xwd", "xdm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Range(&[0x01], &[0x20]),
                        Token::WildcardCount(12),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x05])],
                            &[Token::Literal(&[0x06])],
                            &[Token::Literal(&[0x07])],
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x09])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0B])],
                            &[Token::Literal(&[0x0C])],
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0E])],
                            &[Token::Literal(&[0x0F])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Range(&[0x00], &[0x05]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
