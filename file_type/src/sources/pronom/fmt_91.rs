use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_91: FileFormat = FileFormat {
    id: 634,
    puid: "fmt/91",
    name: "Scalable Vector Graphics",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x3C, 0x73, 0x76, 0x67]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x73, 0x76, 0x67, 0x3E]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 635,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_191,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 635,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_191,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
