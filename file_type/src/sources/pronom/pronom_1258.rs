use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1258: FileFormat = FileFormat {
    id: 1_258,
    source_type: SourceType::Pronom,
    name: "Hypertext Markup Language",
    extensions: &["htm", "html"],
    media_types: &["text/html"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x21]),
                        Token::Any(&[
                            &[Token::Literal(&[0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45])],
                            &[Token::Literal(&[0x64, 0x6F, 0x63, 0x74, 0x79, 0x70, 0x65])],
                        ]),
                        Token::WildcardCountRange(0, 4),
                        Token::Any(&[
                            &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                            &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                        ]),
                        Token::Literal(&[0x3E]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x21]),
                        Token::Any(&[
                            &[Token::Literal(&[0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45])],
                            &[Token::Literal(&[0x64, 0x6F, 0x63, 0x74, 0x79, 0x70, 0x65])],
                        ]),
                        Token::WildcardCountRange(0, 4),
                        Token::Any(&[
                            &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                            &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                        ]),
                        Token::WildcardCountRange(0, 4),
                        Token::Any(&[
                            &[Token::Literal(&[0x53, 0x59, 0x53, 0x54, 0x45, 0x4D])],
                            &[Token::Literal(&[0x73, 0x79, 0x73, 0x74, 0x65, 0x6D])],
                        ]),
                        Token::WildcardCountRange(0, 4),
                        Token::Any(&[&[Token::Literal(&[0x27])], &[Token::Literal(&[0x22])]]),
                        Token::Literal(&[
                            0x61, 0x62, 0x6F, 0x75, 0x74, 0x3A, 0x6C, 0x65, 0x67, 0x61, 0x63, 0x79,
                            0x2D, 0x63, 0x6F, 0x6D, 0x70, 0x61, 0x74,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x27])], &[Token::Literal(&[0x22])]]),
                        Token::Literal(&[0x3E]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 820,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_029,
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
            relationship_type: RelationshipType::HasPriorityOver,
            id: 645,
        },
    ],
};
