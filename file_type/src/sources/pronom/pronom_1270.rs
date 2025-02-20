use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1270: FileType = FileType {
    file_format: &FileFormat {
        id: 1_270,
        source_type: SourceType::Pronom,
        name: "ePub format",
        extensions: &["epub"],
        media_types: &["application/epub+zip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                        Token::WildcardCount(26),
                        Token::Literal(&[
                            0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70, 0x6C,
                            0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x65, 0x70, 0x75, 0x62,
                            0x2B, 0x7A, 0x69, 0x70,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
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
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_258,
            },
        ],
    },
};
