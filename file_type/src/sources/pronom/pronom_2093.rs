use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2093: FileType = FileType {
    file_format: &FileFormat {
        id: 2_093,
        source_type: SourceType::Pronom,
        name: "3M Printscape",
        extensions: &["psc"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x49, 0x49]),
                            Token::WildcardCountRange(0, 320),
                            Token::Literal(&[
                                0x33, 0x4D, 0x20, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x73, 0x63, 0x61,
                                0x70, 0x65,
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
                        tokens: &[
                            Token::Literal(&[0x4D, 0x4D]),
                            Token::WildcardCountRange(0, 320),
                            Token::Literal(&[
                                0x33, 0x4D, 0x20, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x73, 0x63, 0x61,
                                0x70, 0x65,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
