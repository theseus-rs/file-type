use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_205: FileType = FileType {
    file_format: &FileFormat {
        id: 205,
        source_type: SourceType::Pronom,
        name: "Corel Photo-Paint Image",
        extensions: &["cpt"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x43, 0x50, 0x54, 0x37, 0x46, 0x49, 0x4C, 0x45,
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
                            0x43, 0x50, 0x54, 0x38, 0x46, 0x49, 0x4C, 0x45,
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
                            0x43, 0x50, 0x54, 0x39, 0x46, 0x49, 0x4C, 0x45,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
