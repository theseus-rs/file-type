use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1568: FileFormat = FileFormat {
    id: 2_393,
    puid: "fmt/1568",
    name: "ISDOCX Information System Document",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x02])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(12),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x69, 0x73, 0x64, 0x6F, 0x63])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_396,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_533,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_392,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_392,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
