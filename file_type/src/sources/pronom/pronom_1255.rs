use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1255: FileType = FileType {
    file_format: &FileFormat {
        id: 1_255,
        source_type: SourceType::Pronom,
        name: "ISO 9660 Disk Image File",
        extensions: &["iso", "toast", "cdr", "dmg", "bin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(32_700),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x43, 0x44, 0x30, 0x30, 0x31]),
                        Token::WildcardCountRange(1, 16_384),
                        Token::Literal(&[0xFF, 0x43, 0x44, 0x30, 0x30, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_603,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_585,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_586,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_587,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_589,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 2_584,
            },
        ],
    },
};
