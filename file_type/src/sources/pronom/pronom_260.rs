use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_260: FileType = FileType {
    file_format: &FileFormat {
        id: 260,
        source_type: SourceType::Pronom,
        name: "Painter RIFF Image File",
        extensions: &["rif"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x02, 0x00, 0x00]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x3F, 0xE6, 0x66, 0x66]),
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
                            Token::Literal(&[0x00, 0x02, 0x20, 0x00]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x3F, 0xE6, 0x66, 0x66]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
