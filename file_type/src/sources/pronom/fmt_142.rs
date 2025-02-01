use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_142: FileFormat = FileFormat {
    id: 785,
    puid: "fmt/142",
    name: "Waveform Audio (WAVEFORMATEX)",
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
                    Token::NotLiteral(&[0x10]),
                    Token::WildcardCountRange(21, 4_294_967_295),
                    Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 786,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_502,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_503,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_505,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_506,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_507,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 654,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 786,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 654,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 784,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 654,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
