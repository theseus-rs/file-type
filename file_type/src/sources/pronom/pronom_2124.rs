use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2124: FileType = FileType {
    file_format: &FileFormat {
        id: 2_124,
        source_type: SourceType::Pronom,
        name: "LocoScript Document",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4A, 0x4F, 0x59, 0x01]),
                        Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
