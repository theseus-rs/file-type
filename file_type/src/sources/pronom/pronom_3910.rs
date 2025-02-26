use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3910: FileType = FileType {
    file_format: &FileFormat {
        id: 3_910,
        source_type: SourceType::Pronom,
        name: "Plextalk Project File (imph)",
        extensions: &["imph"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                            0xFF, 0xFF, 0xFF, 0xFF,
                        ]),
                        Token::WildcardCount(40),
                        Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF]),
                        Token::WildcardCount(40),
                        Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
