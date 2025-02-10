use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_674: FileType = FileType {
    file_format: &FileFormat {
        id: 674,
        source_type: SourceType::Pronom,
        name: "Exchangeable Image File Format (Audio)",
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
                        Token::Literal(&[0x66, 0x6D, 0x74, 0x20]),
                        Token::WildcardCount(4),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x07])],
                            &[Token::Literal(&[0x11])],
                        ]),
                        Token::Literal(&[0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x4C, 0x49, 0x53, 0x54]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x65, 0x78, 0x69, 0x66, 0x65, 0x76, 0x65, 0x72]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x30, 0x32, 0x31, 0x30]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 654,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 749,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 750,
            },
        ],
    },
};
