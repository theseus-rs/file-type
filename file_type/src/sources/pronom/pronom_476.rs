use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_476: FileType = FileType {
    file_format: &FileFormat {
        id: 476,
        source_type: SourceType::Pronom,
        name: "ESRI Arc/View Project",
        extensions: &["apr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x2F, 0x32, 0x2E]),
                        Token::Range(&[0x30], &[0x33]),
                        Token::Any(&[&[Token::Literal(&[0x0D, 0x0A])], &[Token::Literal(&[0x0A])]]),
                        Token::Literal(&[
                            0x28, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x2E, 0x31,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x0D, 0x0A])], &[Token::Literal(&[0x0A])]]),
                        Token::Literal(&[0x09, 0x4E, 0x61, 0x6D, 0x65, 0x3A, 0x09, 0x22]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
