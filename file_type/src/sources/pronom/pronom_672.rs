use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_672: FileFormat = FileFormat {
    id: 672,
    source_type: SourceType::Pronom,
    name: "Exchangeable Image File Format (Uncompressed)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    signatures: &[
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
                            0x90, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x04, 0x30, 0x32, 0x32, 0x30,
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
                        tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x90, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x32, 0x32, 0x30,
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
                    position_type: PositionType::BOF,
                    offset: Some(10),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x90, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x04, 0x30, 0x32, 0x32, 0x30,
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
                        tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(10),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x90, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x32, 0x32, 0x30,
                        ])],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 795,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 797,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 798,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 799,
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
            id: 1_529,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_867,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_868,
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
            id: 609,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 610,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 611,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 612,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 673,
        },
    ],
};
