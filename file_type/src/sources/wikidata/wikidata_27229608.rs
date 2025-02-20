use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27229608: FileType = FileType {
    file_format: &FileFormat {
        id: 27_229_608,
        source_type: SourceType::Wikidata,
        name: "Portable Network Graphics, version 1.1",
        extensions: &["png"],
        media_types: &["image/png"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00,
                                0x0D, 0x49, 0x48, 0x44, 0x52,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x73, 0x52, 0x47, 0x42]),
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
                            Token::Literal(&[
                                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00,
                                0x0D, 0x49, 0x48, 0x44, 0x52,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x73, 0x50, 0x4C, 0x54]),
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
                            Token::Literal(&[
                                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00,
                                0x0D, 0x49, 0x48, 0x44, 0x52,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x69, 0x43, 0x43, 0x50]),
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
                            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
