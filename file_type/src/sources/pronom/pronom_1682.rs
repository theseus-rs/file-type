use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1682: FileType = FileType {
    file_format: &FileFormat {
        id: 1_682,
        source_type: SourceType::Pronom,
        name: "Corel Presentation",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x0F, 0x0F, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
