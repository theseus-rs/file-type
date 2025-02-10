use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1273: FileType = FileType {
    file_format: &FileFormat {
        id: 1_273,
        source_type: SourceType::Pronom,
        name: "Macromedia (Adobe) Director Compressed Resource file",
        extensions: &["dcr"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x46, 0x47, 0x44, 0x4D]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x58, 0x46, 0x49, 0x52]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x4D, 0x44, 0x47, 0x46]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
