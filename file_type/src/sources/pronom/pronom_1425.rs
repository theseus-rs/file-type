use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1425: FileType = FileType {
    file_format: &FileFormat {
        id: 1_425,
        source_type: SourceType::Pronom,
        name: "LHA File Format",
        extensions: &["lha", "lzh"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x2D, 0x6C, 0x68]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x34])],
                            &[Token::Literal(&[0x35])],
                            &[Token::Literal(&[0x36])],
                            &[Token::Literal(&[0x37])],
                            &[Token::Literal(&[0x64])],
                        ]),
                        Token::Literal(&[0x2D]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
