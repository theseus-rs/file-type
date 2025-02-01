use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1259: FileFormat = FileFormat {
    id: 2_077,
    puid: "fmt/1259",
    name: "SketchUp Document",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0xFF, 0xFE, 0xFF, 0x0E, 0x53, 0x00, 0x6B, 0x00, 0x65, 0x00, 0x74, 0x00,
                        0x63, 0x00, 0x68, 0x00, 0x55, 0x00, 0x70, 0x00, 0x20, 0x00, 0x4D, 0x00,
                        0x6F, 0x00, 0x64, 0x00, 0x65, 0x00, 0x6C, 0x00, 0xFF, 0xFE, 0xFF,
                    ]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x7B, 0x00, 0x31]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_085,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_086,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_087,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_088,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_089,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_090,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_091,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 866,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 867,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
