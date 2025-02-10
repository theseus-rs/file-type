use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1118: FileType = FileType {
    file_format: &FileFormat {
        id: 1_118,
        source_type: SourceType::Pronom,
        name: "Enhanced Compression Wavelet",
        extensions: &["ecw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x65, 0x02, 0x01, 0x02]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x04, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x06])],
                        ]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x06])],
                        ]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x03, 0x00, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
