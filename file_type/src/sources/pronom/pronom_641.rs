use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_641: FileType = FileType {
    file_format: &FileFormat {
        id: 641,
        source_type: SourceType::Pronom,
        name: "Hypertext Markup Language",
        extensions: &["htm", "html"],
        media_types: &["text/html"],
        signatures: &[Signature {
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
                        Token::Literal(&[0x20, 0x34, 0x2E]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30, 0x20])],
                            &[Token::Literal(&[0x30, 0x2F])],
                        ]),
                    ],
                },
            }],
        }],
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
                id: 820,
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
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_018,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 642,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 640,
            },
        ],
    },
};
