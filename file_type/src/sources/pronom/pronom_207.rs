use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_207: FileType = FileType {
    file_format: &FileFormat {
        id: 207,
        source_type: SourceType::Pronom,
        name: "Scitex Continuous Tone Bitmap",
        extensions: &["ct", "sct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(80),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x43, 0x54, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00,
                        ]),
                        Token::SingleWildcard,
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00,
                        ]),
                        Token::WildcardCount(879),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x07])],
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0xFF])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
