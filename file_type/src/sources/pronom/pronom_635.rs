use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_635: FileType = FileType {
    file_format: &FileFormat {
        id: 635,
        source_type: SourceType::Pronom,
        name: "Scalable Vector Graphics",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x3C, 0x73, 0x76, 0x67]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x31,
                            0x22,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x73, 0x76, 0x67, 0x3E]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 316,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 634,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_191,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 634,
            },
        ],
    },
};
