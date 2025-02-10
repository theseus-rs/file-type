use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1346: FileType = FileType {
    file_format: &FileFormat {
        id: 1_346,
        source_type: SourceType::Pronom,
        name: "Adobe Illustrator",
        extensions: &["ai", "pdf"],
        media_types: &["application/postscript"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E]),
                            Token::Any(&[&[Token::Literal(&[0x33])], &[Token::Literal(&[0x34])]]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x41, 0x49, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x44, 0x61,
                                0x74, 0x61,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x33, 0x2E, 0x30,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F,
                                0x72, 0x6D, 0x61, 0x74, 0x20,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x34])], &[Token::Literal(&[0x35])]]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                                0x33, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(128, 1_024),
                            Token::Literal(&[
                                0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F,
                                0x72, 0x6D, 0x61, 0x74, 0x20, 0x35,
                            ]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 616,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 617,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_347,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_345,
            },
        ],
    },
};
