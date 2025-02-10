use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_14: FileType = FileType {
    file_format: &FileFormat {
        id: 14,
        source_type: SourceType::Pronom,
        name: "Write for Windows Document",
        extensions: &["wri"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x32, 0xBE, 0x00, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00,
                        ]),
                        Token::WildcardCount(82),
                        Token::NotLiteral(&[0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
