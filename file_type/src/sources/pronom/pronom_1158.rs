use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1158: FileType = FileType {
    file_format: &FileFormat {
        id: 1_158,
        source_type: SourceType::Pronom,
        name: "Internet Archive",
        extensions: &["arc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x66, 0x69, 0x6C, 0x65, 0x64, 0x65, 0x73, 0x63, 0x3A, 0x2F, 0x2F,
                        ]),
                        Token::WildcardCountRange(1, 132),
                        Token::Literal(&[0x20]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x2E]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x2E]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x2E]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x20]),
                        Token::WildcardCount(14),
                        Token::Literal(&[
                            0x20, 0x74, 0x65, 0x78, 0x74, 0x2F, 0x70, 0x6C, 0x61, 0x69, 0x6E, 0x20,
                        ]),
                        Token::WildcardCountRange(1, 16),
                        Token::Literal(&[0x0A, 0x31, 0x20, 0x31, 0x20]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x0A, 0x55, 0x52, 0x4C, 0x20, 0x49, 0x50, 0x2D, 0x61, 0x64, 0x64, 0x72,
                            0x65, 0x73, 0x73, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x2D,
                            0x64, 0x61, 0x74, 0x65, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74,
                            0x2D, 0x74, 0x79, 0x70, 0x65, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76,
                            0x65, 0x2D, 0x6C, 0x65, 0x6E, 0x67, 0x74, 0x68,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 639,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 640,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 641,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 642,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 643,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 644,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 645,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 310,
            },
        ],
    },
};
