use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2281: FileType = FileType {
    file_format: &FileFormat {
        id: 2_281,
        source_type: SourceType::Pronom,
        name: "Arts & Letters Graphics File",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x41, 0x72, 0x74, 0x73, 0x20, 0x26, 0x20, 0x4C, 0x65, 0x74, 0x74, 0x65,
                            0x72, 0x73, 0x20,
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
                            0x41, 0x26, 0x4C, 0x2D, 0x00, 0x41, 0x72, 0x74, 0x73, 0x20, 0x26, 0x20,
                            0x4C, 0x65, 0x74, 0x74, 0x65, 0x72, 0x73, 0x20,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
