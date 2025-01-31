use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_468: FileFormat = FileFormat {
    id: 1_255,
    puid: "fmt/468",
    name: "ISO 9660 Disk Image File",
    extensions: &["iso", "toast", "cdr", "dmg", "bin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
            id: 1_603,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_585,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_586,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_587,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_589,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_584,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
