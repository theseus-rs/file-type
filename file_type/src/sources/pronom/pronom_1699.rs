use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1699: FileType = FileType {
    file_format: &FileFormat {
        id: 1_699,
        source_type: SourceType::Pronom,
        name: "JEOL NMR Spectroscopy",
        extensions: &["jdf"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4A, 0x45, 0x4F, 0x4C, 0x2E, 0x4E, 0x4D, 0x52,
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
                            0x52, 0x4D, 0x4E, 0x2E, 0x4C, 0x4F, 0x45, 0x4A,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
