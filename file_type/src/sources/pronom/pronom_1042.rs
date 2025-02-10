use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1042: FileType = FileType {
    file_format: &FileFormat {
        id: 1_042,
        source_type: SourceType::Pronom,
        name: "Autodesk Animator Pro FLIC",
        extensions: &["flc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x12, 0xAF]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x08]),
                        Token::WildcardCount(120),
                        Token::Literal(&[0xF1]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
