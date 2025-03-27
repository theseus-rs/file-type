use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27596100: FileType = FileType {
    file_format: &FileFormat {
        id: 27_596_100,
        source_type: SourceType::Wikidata,
        name: "Windows Bitmap, version 1",
        extensions: &["bmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x01]),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
