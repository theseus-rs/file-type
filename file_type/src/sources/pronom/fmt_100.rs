use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_100: FileFormat = FileFormat {
    id: 642,
    puid: "fmt/100",
    name: "Hypertext Markup Language",
    extensions: &["htm", "html"],
    media_types: &["text/html"],
    internal_signatures: &[InternalSignature {
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
                    Token::Literal(&[0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                        &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                    ]),
                    Token::Literal(&[0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[0x50, 0x55, 0x42, 0x4C, 0x49, 0x43])],
                        &[Token::Literal(&[0x70, 0x75, 0x62, 0x6C, 0x69, 0x63])],
                    ]),
                    Token::Literal(&[0x20, 0x22, 0x2D, 0x2F, 0x2F]),
                    Token::WildcardCountRange(1, 16),
                    Token::Literal(&[0x2F, 0x2F]),
                    Token::Any(&[
                        &[Token::Literal(&[0x44, 0x54, 0x44])],
                        &[Token::Literal(&[0x64, 0x74, 0x64])],
                    ]),
                    Token::Literal(&[0x20]),
                    Token::WildcardCountRange(0, 64),
                    Token::Any(&[
                        &[Token::Literal(&[0x48, 0x54, 0x4D, 0x4C])],
                        &[Token::Literal(&[0x68, 0x74, 0x6D, 0x6C])],
                    ]),
                    Token::Literal(&[0x20, 0x34, 0x2E, 0x30, 0x31]),
                ],
            },
        }],
    }],
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
            id: 820,
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
        RelatedFormat {
            id: 645,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 641,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
