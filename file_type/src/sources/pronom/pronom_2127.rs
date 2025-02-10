use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2127: FileType = FileType {
    file_format: &FileFormat {
        id: 2_127,
        source_type: SourceType::Pronom,
        name: "LocoScript Professional",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x44, 0x4F, 0x43, 0x01]),
                        Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x03])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
