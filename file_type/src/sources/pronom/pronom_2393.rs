use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2393: FileType = FileType {
    file_format: &FileFormat {
        id: 2_393,
        source_type: SourceType::Pronom,
        name: "ISDOCX Information System Document",
        extensions: &["isdocx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x02])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(12),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x2E, 0x69, 0x73, 0x64, 0x6F, 0x63])],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_396,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_533,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_392,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_392,
            },
        ],
    },
};
