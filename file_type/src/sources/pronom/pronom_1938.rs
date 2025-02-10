use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1938: FileType = FileType {
    file_format: &FileFormat {
        id: 1_938,
        source_type: SourceType::Pronom,
        name: "Progressive Graphics File",
        extensions: &["pgf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x47, 0x46]),
                        Token::Any(&[
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x05])],
                            &[Token::Literal(&[0x06])],
                            &[Token::Literal(&[0x07])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
