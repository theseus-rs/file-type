use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3892: FileType = FileType {
    file_format: &FileFormat {
        id: 3_892,
        source_type: SourceType::Pronom,
        name: "Sony OpenMG Audio",
        extensions: &["oma"],
        media_types: &["audio/ATRAC-ADVANCED-LOSSLESS"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x65, 0x61, 0x33]),
                            Token::WildcardCount(3_069),
                            Token::Literal(&[0x45, 0x41, 0x33]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(8),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x6F, 0x70, 0x65, 0x6E, 0x4D, 0x47])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
