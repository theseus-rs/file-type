use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1774: FileType = FileType {
    file_format: &FileFormat {
        id: 1_774,
        source_type: SourceType::Pronom,
        name: "Rich Text Format",
        extensions: &["rtf"],
        media_types: &["application/rtf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x7B, 0x5C, 0x72, 0x74, 0x66, 0x30, 0x5C]),
                        Token::Any(&[
                            &[Token::Literal(&[0x61, 0x6E, 0x73, 0x69])],
                            &[Token::Literal(&[0x6D, 0x61, 0x63])],
                            &[Token::Literal(&[0x70, 0x63])],
                            &[Token::Literal(&[0x70, 0x63, 0x61])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
