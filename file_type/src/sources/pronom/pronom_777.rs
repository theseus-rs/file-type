use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_777: FileType = FileType {
    file_format: &FileFormat {
        id: 777,
        source_type: SourceType::Pronom,
        name: "Java Archive Format",
        extensions: &["jar"],
        media_types: &["application/java-archive"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x4D, 0x45, 0x54, 0x41, 0x2D, 0x49, 0x4E, 0x46, 0x2F, 0x4D, 0x41,
                                0x4E, 0x49, 0x46, 0x45, 0x53, 0x54, 0x2E, 0x4D, 0x46,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(18),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x4B, 0x05, 0x06])],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 778,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 783,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_206,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_231,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
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
        ],
    },
};
