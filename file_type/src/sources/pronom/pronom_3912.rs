use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3912: FileType = FileType {
    file_format: &FileFormat {
        id: 3_912,
        source_type: SourceType::Pronom,
        name: "Plextalk Project File (imtt)",
        extensions: &["imtt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::WildcardCountRange(22, 24),
                        Token::Literal(&[0x03, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x0B, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
