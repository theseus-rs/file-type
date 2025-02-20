use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_736: FileType = FileType {
    file_format: &FileFormat {
        id: 736,
        source_type: SourceType::Pronom,
        name: "WordPerfect for MS-DOS Document",
        extensions: &["wp", "wp5", "wpd", "w50", "doc"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(5),
                        Token::Literal(&[0x0A, 0x00, 0x00]),
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
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 737,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_754,
            },
        ],
    },
};
