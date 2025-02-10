use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1527: FileType = FileType {
    file_format: &FileFormat {
        id: 1_527,
        source_type: SourceType::Pronom,
        name: "RealLegal E-Transcript",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(10),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x00, 0x00, 0x02])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::Variable,
                        offset: None,
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x00, 0x54, 0x68, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6E, 0x73,
                                0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74,
                                0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x64, 0x65, 0x6C, 0x69,
                                0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x0D, 0x0A, 0x3C,
                                0x43, 0x4F, 0x4D, 0x50, 0x41, 0x4E, 0x59, 0x3E, 0x2E, 0x0D, 0x0A,
                                0x50, 0x68, 0x3A,
                            ])],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x00, 0x00, 0x03])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::Variable,
                        offset: None,
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x00, 0x54, 0x68, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6E, 0x73,
                                0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74,
                                0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x64, 0x65, 0x6C, 0x69,
                                0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x0D, 0x0A, 0x3C,
                                0x43, 0x4F, 0x4D, 0x50, 0x41, 0x4E, 0x59, 0x3E, 0x2E, 0x0D, 0x0A,
                                0x50, 0x68, 0x3A,
                            ])],
                        },
                    },
                ],
            },
        ],
        related_formats: &[],
    },
};
