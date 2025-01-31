use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_96: FileFormat = FileFormat {
    id: 645,
    puid: "fmt/96",
    name: "Hypertext Markup Language",
    extensions: &["htm", "html"],
    media_types: &["text/html"],
    internal_signatures: &[
        InternalSignature {
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
        InternalSignature {
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
            id: 310,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 639,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 640,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 641,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 642,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 643,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 644,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 777,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_018,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_029,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_158,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_258,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_269,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_270,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_287,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_371,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_670,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_099,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_173,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
