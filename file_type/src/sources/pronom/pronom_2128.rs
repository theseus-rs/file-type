use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2128: FileType = FileType {
    file_format: &FileFormat {
        id: 2_128,
        source_type: SourceType::Pronom,
        name: "LocoFile",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x49, 0x4C, 0x01]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
