use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1676: FileType = FileType {
    file_format: &FileFormat {
        id: 1_676,
        source_type: SourceType::Pronom,
        name: "Free Lossless Image Format (FLIF)",
        extensions: &["flif"],
        media_types: &["image/flif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4C, 0x49, 0x46]),
                        Token::Any(&[
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x33])],
                            &[Token::Literal(&[0x34])],
                            &[Token::Literal(&[0x41])],
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x51])],
                            &[Token::Literal(&[0x53])],
                            &[Token::Literal(&[0x54])],
                            &[Token::Literal(&[0x61])],
                            &[Token::Literal(&[0x63])],
                            &[Token::Literal(&[0x64])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
