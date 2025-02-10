use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2040: FileType = FileType {
    file_format: &FileFormat {
        id: 2_040,
        source_type: SourceType::Pronom,
        name: "NMRPipe",
        extensions: &["dat", "pipe", "ft2", "ft3"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x89, 0x88, 0x88, 0xCD, 0x7B, 0x14, 0x16, 0x40,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x4F, 0x6E, 0xEE, 0xEF, 0x40, 0x16, 0x14, 0x7B,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
