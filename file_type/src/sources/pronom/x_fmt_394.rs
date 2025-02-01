use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_394: FileFormat = FileFormat {
    id: 737,
    puid: "x-fmt/394",
    name: "WordPerfect for MS-DOS/Windows Document",
    extensions: &["wp5", "wpd", "w51", "wp", "doc"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[InternalSignature {
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
            id: 2_702,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_098,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 75,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 281,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 736,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
