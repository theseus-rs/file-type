use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_785: FileFormat = FileFormat {
    id: 785,
    source_type: SourceType::Pronom,
    name: "Waveform Audio (WAVEFORMATEX)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x66, 0x6D, 0x74, 0x20]),
                    Token::NotLiteral(&[0x10]),
                    Token::WildcardCountRange(21, 4_294_967_295),
                    Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 786,
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
            id: 1_505,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_506,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_507,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 654,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 786,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 654,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 784,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 654,
        },
    ],
};
