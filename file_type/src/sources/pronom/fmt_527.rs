use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_527: FileFormat = FileFormat {
    id: 1_314,
    puid: "fmt/527",
    name: "Broadcast WAVE",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x57, 0x41, 0x56, 0x45]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x62, 0x65, 0x78, 0x74]),
                    Token::WildcardCount(350),
                    Token::Literal(&[0x02, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_504,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_507,
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
            id: 784,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 786,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 656,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
