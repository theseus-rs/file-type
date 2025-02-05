use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_737: FileFormat = FileFormat {
    id: 737,
    source_type: SourceType::Pronom,
    name: "WordPerfect for MS-DOS/Windows Document",
    extensions: &["wp5", "wpd", "w51", "wp", "doc"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x0A, 0x00, 0x01]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_702,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_098,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 75,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 281,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 736,
        },
    ],
};
