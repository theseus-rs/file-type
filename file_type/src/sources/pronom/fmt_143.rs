use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_143: FileFormat = FileFormat {
    id: 786,
    puid: "fmt/143",
    name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
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
                    Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x66, 0x6D, 0x74, 0x20]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0xFE, 0xFF]),
                    Token::WildcardCountRange(38, 4_294_967_295),
                    Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 656,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 735,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_314,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_508,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_509,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_510,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 654,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 785,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 785,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 654,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
