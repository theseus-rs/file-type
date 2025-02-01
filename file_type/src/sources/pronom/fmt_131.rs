use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_131: FileFormat = FileFormat {
    id: 691,
    puid: "fmt/131",
    name: "Advanced Systems Format",
    extensions: &["asf"],
    media_types: &["application/vnd.ms-asf"],
    internal_signatures: &[InternalSignature {
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
            id: 692,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 693,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_228,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 692,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 693,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
