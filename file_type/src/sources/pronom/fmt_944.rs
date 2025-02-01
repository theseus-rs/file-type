use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_944: FileFormat = FileFormat {
    id: 1_749,
    puid: "fmt/944",
    name: "Ogg Multimedia Container",
    extensions: &["ogg", "ogv", "spx", "opus"],
    media_types: &["application/ogg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 929,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 1_750,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 1_751,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 1_752,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 1_753,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 929,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_750,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_751,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_752,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_753,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
