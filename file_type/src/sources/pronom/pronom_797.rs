use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_797: FileType = FileType {
    file_format: &FileFormat {
        id: 797,
        source_type: SourceType::Pronom,
        name: "Tagged Image File Format for Electronic Photography (TIFF/EP)",
        extensions: &["tif", "tiff"],
        media_types: &["image/tiff"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::WildcardCountRange(6, 4_080),
                            Token::Literal(&[
                                0x16, 0x92, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x01,
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
                        offset: Some(4),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x16, 0x92, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x01,
                            ])],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::WildcardCountRange(6, 4_080),
                            Token::Literal(&[
                                0x92, 0x16, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x00, 0x00,
                                0x00,
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
                            tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(4),
                        regex: Regex {
                            tokens: &[Token::Literal(&[
                                0x92, 0x16, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x00, 0x00,
                                0x00,
                            ])],
                        },
                    },
                ],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 927,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_223,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_224,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_225,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_440,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_529,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_693,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_694,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_804,
            },
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
                id: 1_099,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 612,
            },
        ],
    },
};
