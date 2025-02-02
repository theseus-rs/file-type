use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_784: FileFormat = FileFormat {
    id: 784,
    source_type: SourceType::Pronom,
    name: "Waveform Audio (PCMWAVEFORMAT)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[
                        0x57, 0x41, 0x56, 0x45, 0x66, 0x6D, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00,
                        0x01, 0x00,
                    ]),
                    Token::WildcardCountRange(14, 4_294_967_295),
                    Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 656,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 735,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_314,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_502,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_503,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_504,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 654,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 785,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 654,
        },
    ],
};
