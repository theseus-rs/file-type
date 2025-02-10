use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2077: FileType = FileType {
    file_format: &FileFormat {
        id: 2_077,
        source_type: SourceType::Pronom,
        name: "SketchUp Document",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0xFF, 0xFE, 0xFF, 0x0E, 0x53, 0x00, 0x6B, 0x00, 0x65, 0x00, 0x74, 0x00,
                            0x63, 0x00, 0x68, 0x00, 0x55, 0x00, 0x70, 0x00, 0x20, 0x00, 0x4D, 0x00,
                            0x6F, 0x00, 0x64, 0x00, 0x65, 0x00, 0x6C, 0x00, 0xFF, 0xFE, 0xFF,
                        ]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x7B, 0x00, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_085,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_086,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_087,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_088,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_089,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_090,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_091,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 866,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 867,
            },
        ],
    },
};
