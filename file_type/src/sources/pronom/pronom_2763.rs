use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2763: FileType = FileType {
    file_format: &FileFormat {
        id: 2_763,
        source_type: SourceType::Pronom,
        name: "Micrografx Icon File",
        extensions: &["icn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x5A, 0x5A, 0x5A, 0x5A]),
                        Token::WildcardCount(37),
                        Token::Literal(&[0x76, 0x69, 0x65, 0x77]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
