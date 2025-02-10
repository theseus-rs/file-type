use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_75: FileType = FileType {
    file_format: &FileFormat {
        id: 75,
        source_type: SourceType::Pronom,
        name: "WordPerfect for MS-DOS/Windows Document",
        extensions: &["doc", "wpd", "wp6", "wp", "w60", "w61", "w62"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x01, 0x0A, 0x02]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 281,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 737,
            },
        ],
    },
};
