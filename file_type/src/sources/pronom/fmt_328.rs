use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_328: FileFormat = FileFormat {
    id: 1_073,
    puid: "fmt/328",
    name: "EndNote Import File",
    extensions: &["enw", "enr"],
    media_types: &["application/x-endnote-refer"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x41, 0x20]),
                        Token::Range(&[0x41], &[0x5A]),
                        Token::WildcardCountRange(5, 50),
                        Token::Any(&[&[Token::Literal(&[0x0A])], &[Token::Literal(&[0x0A, 0x0D])]]),
                        Token::Literal(&[0x25]),
                        Token::Any(&[
                            &[Token::Literal(&[0x41])],
                            &[Token::Literal(&[0x42])],
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x54])],
                        ]),
                        Token::Literal(&[0x20]),
                        Token::Any(&[
                            &[Token::Range(&[0x30], &[0x39])],
                            &[Token::Range(&[0x41], &[0x5A])],
                        ]),
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
                        Token::Literal(&[0x25, 0x41]),
                        Token::Range(&[0x41], &[0x5A]),
                        Token::WildcardCountRange(5, 50),
                        Token::Any(&[&[Token::Literal(&[0x0A])], &[Token::Literal(&[0x0A, 0x0D])]]),
                        Token::Literal(&[0x25]),
                        Token::Any(&[
                            &[Token::Literal(&[0x41])],
                            &[Token::Literal(&[0x42])],
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x54])],
                        ]),
                        Token::Any(&[
                            &[Token::Range(&[0x30], &[0x39])],
                            &[Token::Range(&[0x41], &[0x5A])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 86,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 138,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 331,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 332,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 771,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 772,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 773,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_288,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
