use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1740: FileFormat = FileFormat {
    id: 2_586,
    puid: "fmt/1740",
    name: "Apple Partition Map Disk Image",
    extensions: &["toast", "iso", "cdr", "dmg", "bin", "img"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x45, 0x52, 0x02, 0x00]),
                        Token::WildcardCount(508),
                        Token::Literal(&[0x50, 0x4D]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x45, 0x52, 0x08, 0x00]),
                        Token::WildcardCount(2_044),
                        Token::Literal(&[0x50, 0x4D]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(508),
                        Token::Literal(&[0x50, 0x4D]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_878,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_587,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_605,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_255,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
