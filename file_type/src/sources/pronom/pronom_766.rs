use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_766: FileType = FileType {
    file_format: &FileFormat {
        id: 766,
        source_type: SourceType::Pronom,
        name: "Drawing Interchange File Format (ASCII)",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x30]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D])],
                            ]),
                            Token::Literal(&[0x45, 0x4F, 0x46]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x30]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D])],
                            ]),
                            Token::Literal(&[0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D])],
                            ]),
                            Token::WildcardCountRange(0, 5),
                            Token::Literal(&[0x32]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D])],
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x30]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0D])],
                            ]),
                            Token::Literal(&[0x45, 0x4E, 0x44, 0x53, 0x45, 0x43]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 710,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 711,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 712,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 713,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 714,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 715,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 716,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 717,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 718,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 719,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 720,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 721,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 722,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 723,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 724,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 725,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_220,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_221,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_222,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_319,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_207,
            },
        ],
    },
};
