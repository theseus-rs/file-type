use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_157: FileFormat = FileFormat {
    id: 221,
    puid: "x-fmt/157",
    name: "Interchange File",
    extensions: &["iff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Any(&[
                        &[Token::Literal(&[0x46, 0x4F, 0x52, 0x4D])],
                        &[Token::Literal(&[0x4C, 0x49, 0x53, 0x54])],
                        &[Token::Literal(&[0x43, 0x41, 0x54, 0x20])],
                    ]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 195,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_083,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_084,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_192,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_194,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_015,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_016,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_831,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_083,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_084,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
