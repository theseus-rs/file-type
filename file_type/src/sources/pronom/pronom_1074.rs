use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1074: FileType = FileType {
    file_format: &FileFormat {
        id: 1_074,
        source_type: SourceType::Pronom,
        name: "Shell Archive Format",
        extensions: &["shar"],
        media_types: &["application/x-sh", "application/x-shar"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x23, 0x21]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[
                                0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x73, 0x68, 0x0A, 0x23, 0x20, 0x54,
                                0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x68,
                                0x65, 0x6C, 0x6C, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                            ]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x23, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
                            0x73, 0x68, 0x65, 0x6C, 0x6C, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76,
                            0x65,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
