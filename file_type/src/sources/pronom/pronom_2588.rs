use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2588: FileType = FileType {
    file_format: &FileFormat {
        id: 2_588,
        source_type: SourceType::Pronom,
        name: "Hierarchical File System Plus",
        extensions: &["img", "dmg", "toast"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(1_024),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x48, 0x2B, 0x00, 0x04])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x48, 0x2B, 0x00, 0x04])],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(1_024),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x48, 0x58, 0x00, 0x05])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x48, 0x58, 0x00, 0x05])],
                        },
                    },
                ],
            },
        ],
        related_formats: &[],
    },
};
