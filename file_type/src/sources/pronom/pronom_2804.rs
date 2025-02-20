use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2804: FileType = FileType {
    file_format: &FileFormat {
        id: 2_804,
        source_type: SourceType::Pronom,
        name: "Digital Negative Format (DNG)",
        extensions: &["dng"],
        media_types: &["image/dng"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::WildcardCountRange(0, 4_080),
                            Token::Literal(&[
                                0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x07,
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
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::WildcardCountRange(0, 4_080),
                            Token::Literal(&[
                                0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x07,
                            ]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x12, 0xC6, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x07,
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
                            tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0xC6, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x07,
                            ])],
                        },
                    },
                ],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 672,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 673,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 752,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 797,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_099,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_694,
            },
        ],
    },
};
