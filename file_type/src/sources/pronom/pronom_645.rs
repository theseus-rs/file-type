use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_645: FileType = FileType {
    file_format: &FileFormat {
        id: 645,
        source_type: SourceType::Pronom,
        name: "Hypertext Markup Language",
        extensions: &["htm", "html"],
        media_types: &["text/html"],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[
                                Token::Literal(&[0x3C]),
                                Token::Any(&[
                                    &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                                    &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                                ]),
                            ],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[
                                Token::Literal(&[0x3C, 0x2F]),
                                Token::Any(&[
                                    &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                                    &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                                    &[Token::Literal(&[0x42, 0x4F, 0x44, 0x59])],
                                    &[Token::Literal(&[0x62, 0x6F, 0x64, 0x79])],
                                ]),
                                Token::Literal(&[0x3E]),
                            ],
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
                            tokens: &[
                                Token::Literal(&[0x3C, 0x00]),
                                Token::Any(&[
                                    &[Token::Literal(&[
                                        0x48, 0x00, 0x54, 0x00, 0x4D, 0x00, 0x4C, 0x00,
                                    ])],
                                    &[Token::Literal(&[
                                        0x68, 0x00, 0x74, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                                    ])],
                                ]),
                            ],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[
                                Token::Literal(&[0x3C, 0x00, 0x2F, 0x00]),
                                Token::Any(&[
                                    &[Token::Literal(&[
                                        0x48, 0x00, 0x54, 0x00, 0x4D, 0x00, 0x4C, 0x00,
                                    ])],
                                    &[Token::Literal(&[
                                        0x68, 0x00, 0x74, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                                    ])],
                                    &[Token::Literal(&[
                                        0x42, 0x00, 0x4F, 0x00, 0x44, 0x00, 0x59, 0x00,
                                    ])],
                                    &[Token::Literal(&[
                                        0x62, 0x00, 0x6F, 0x00, 0x64, 0x00, 0x79, 0x00,
                                    ])],
                                ]),
                                Token::Literal(&[0x3E, 0x00]),
                            ],
                        },
                    },
                ],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 310,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 639,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 640,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 641,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 642,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 643,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 644,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 777,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_018,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_029,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_158,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_258,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_269,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_270,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_287,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_371,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_670,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_099,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_173,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_930,
            },
        ],
    },
};
