use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_412: FileFormat = FileFormat {
    id: 777,
    puid: "x-fmt/412",
    name: "Java Archive Format",
    extensions: &["jar"],
    media_types: &["application/java-archive"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x4D, 0x45, 0x54, 0x41, 0x2D, 0x49, 0x4E, 0x46, 0x2F, 0x4D, 0x41, 0x4E,
                            0x49, 0x46, 0x45, 0x53, 0x54, 0x2E, 0x4D, 0x46,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(18),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x05, 0x06])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 783,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_206,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_231,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 643,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 644,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 645,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
