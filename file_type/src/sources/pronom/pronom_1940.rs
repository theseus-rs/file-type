use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1940: FileType = FileType {
    file_format: &FileFormat {
        id: 1_940,
        source_type: SourceType::Pronom,
        name: "C3D File Format",
        extensions: &["c3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x02, 0x50]),
                        Token::WildcardCount(22),
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(481),
                        Token::Any(&[
                            &[Token::Literal(&[0x54])],
                            &[Token::Literal(&[0x55])],
                            &[Token::Literal(&[0x56])],
                        ]),
                        Token::WildcardCount(2),
                        Token::Any(&[
                            &[Token::Literal(&[0x50, 0x4F, 0x49, 0x4E, 0x54])],
                            &[Token::Literal(&[0x41, 0x4E, 0x41, 0x4C, 0x4F, 0x47])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
