use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1756: FileType = FileType {
    file_format: &FileFormat {
        id: 1_756,
        source_type: SourceType::Pronom,
        name: "Sonic Foundry WAVE 64",
        extensions: &["w64", "wav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x72, 0x69, 0x66, 0x66, 0x2E, 0x91, 0xCF, 0x11, 0xA5, 0xD6, 0x28, 0xDB,
                            0x04, 0xC1, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x77, 0x61, 0x76, 0x65, 0xF3, 0xAC, 0xD3, 0x11]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
