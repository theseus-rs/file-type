use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2821: FileType = FileType {
    file_format: &FileFormat {
        id: 2_821,
        source_type: SourceType::Pronom,
        name: "Sandboxels Save File",
        extensions: &["sbxls"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x7B, 0x22, 0x6D, 0x65, 0x74, 0x61, 0x22, 0x3A, 0x7B, 0x22, 0x73, 0x61,
                            0x76, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x3A, 0x22,
                            0x73, 0x62,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x31])], &[Token::Literal(&[0x32])]]),
                        Token::Literal(&[
                            0x22, 0x2C, 0x22, 0x67, 0x61, 0x6D, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69,
                            0x6F, 0x6E, 0x22, 0x3A,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
