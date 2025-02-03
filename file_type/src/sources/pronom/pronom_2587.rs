use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2587: FileFormat = FileFormat {
    id: 2_587,
    source_type: SourceType::Pronom,
    name: "Apple Partition Map ISO 9660 Hybrid",
    extensions: &["toast", "iso", "cdr"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(32_768),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x45, 0x52, 0x02, 0x00]),
                        Token::WildcardCount(508),
                        Token::Literal(&[0x50, 0x4D, 0x43, 0x44, 0x30, 0x30, 0x31]),
                        Token::WildcardCountRange(1, 16_384),
                        Token::Literal(&[0xFF, 0x43, 0x44, 0x30, 0x30, 0x31]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(32_768),
                regex: Regex {
                    tokens: &[
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[0x45, 0x52, 0x08, 0x00]),
                        Token::WildcardCount(2_044),
                        Token::Literal(&[0x50, 0x4D, 0x43, 0x44, 0x30, 0x30, 0x31]),
                        Token::WildcardCountRange(1, 16_384),
                        Token::Literal(&[0xFF, 0x43, 0x44, 0x30, 0x30, 0x31]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_255,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_586,
        },
    ],
};
