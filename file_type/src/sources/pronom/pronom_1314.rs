use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1314: FileType = FileType {
    file_format: &FileFormat {
        id: 1_314,
        source_type: SourceType::Pronom,
        name: "Broadcast WAVE",
        extensions: &["wav"],
        media_types: &["audio/x-wav"],
        signatures: &[Signature {
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_504,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_507,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_510,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 654,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 784,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 786,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 656,
            },
        ],
    },
};
