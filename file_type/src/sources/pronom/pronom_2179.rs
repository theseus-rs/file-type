use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2179: FileType = FileType {
    file_format: &FileFormat {
        id: 2_179,
        source_type: SourceType::Pronom,
        name: "Amiga Disk File",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x44, 0x4F, 0x53]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
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
