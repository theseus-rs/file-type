use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_691: FileFormat = FileFormat {
    id: 691,
    source_type: SourceType::Pronom,
    name: "Advanced Systems Format",
    extensions: &["asf"],
    media_types: &["application/vnd.ms-asf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA,
                        0x00, 0x62, 0xCE, 0x6C,
                    ]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x01, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 692,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 693,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_228,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 692,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 693,
        },
    ],
};
