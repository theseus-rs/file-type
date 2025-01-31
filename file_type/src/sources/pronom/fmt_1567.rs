use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1567: FileFormat = FileFormat {
    id: 2_392,
    puid: "fmt/1567",
    name: "ISDOC Information System Document",
    extensions: &["isdoc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x73, 0x64, 0x6F, 0x63, 0x2E, 0x63, 0x7A,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_393,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_395,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_395,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_393,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
