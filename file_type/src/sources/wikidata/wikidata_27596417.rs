use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27596417: FileType = FileType {
    file_format: &FileFormat {
        id: 27_596_417,
        source_type: SourceType::Wikidata,
        name: "Windows Bitmap, version 5",
        extensions: &["bmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x7C, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x01, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x05])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
