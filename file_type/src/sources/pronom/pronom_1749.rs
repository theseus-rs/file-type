use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1749: FileType = FileType {
    file_format: &FileFormat {
        id: 1_749,
        source_type: SourceType::Pronom,
        name: "Ogg Multimedia Container",
        extensions: &["ogg", "ogv", "spx", "opus"],
        media_types: &["application/ogg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 929,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 1_750,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 1_751,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 1_752,
            },
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 1_753,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 929,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_750,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_751,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_752,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_753,
            },
        ],
    },
};
