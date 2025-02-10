use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_53: FileType = FileType {
    file_format: &FileFormat {
        id: 53,
        source_type: SourceType::Pronom,
        name: "AutoCAD Block Attribute Template",
        extensions: &["blk"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00,
                                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x02, 0x00, 0x00, 0x00,
                            ]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x41, 0x63, 0x44, 0x78]),
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
                            Token::Literal(&[0x01, 0x00, 0x00, 0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[
                                0x00, 0xFF, 0xFF, 0x01, 0x00, 0x0E, 0x00, 0x43, 0x45, 0x61, 0x74,
                                0x74, 0x42, 0x6C, 0x6F, 0x63, 0x6B, 0x49, 0x6E, 0x66, 0x6F,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
