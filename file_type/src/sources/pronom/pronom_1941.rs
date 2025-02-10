use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1941: FileType = FileType {
    file_format: &FileFormat {
        id: 1_941,
        source_type: SourceType::Pronom,
        name: "Gatan Digital Micrograph File Format (DM3)",
        extensions: &["dm3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x00, 0x00, 0x03]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                            Token::WildcardCount(6),
                            Token::Any(&[&[Token::Literal(&[0x14])], &[Token::Literal(&[0x15])]]),
                            Token::WildcardCountRange(2, 258),
                            Token::Literal(&[0x25, 0x25, 0x25, 0x25]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
