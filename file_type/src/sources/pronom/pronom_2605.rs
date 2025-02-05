use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2605: FileFormat = FileFormat {
    id: 2_605,
    source_type: SourceType::Pronom,
    name: "Apple Partition Map - ISO 9660 - UDF Hybrid Disk Image",
    extensions: &["iso", "toast", "dmg"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[
                ByteSequence {
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
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(32_768),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x43, 0x44, 0x30, 0x30, 0x31]),
                            Token::WildcardCountRange(1, 16_384),
                            Token::Literal(&[0xFF, 0x43, 0x44, 0x30, 0x30, 0x31]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(32_768),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x42, 0x45, 0x41, 0x30, 0x31, 0x01]),
                            Token::WildcardCount(2_041),
                            Token::Literal(&[0x00, 0x4E, 0x53, 0x52, 0x30]),
                            Token::Any(&[&[Token::Literal(&[0x32])], &[Token::Literal(&[0x33])]]),
                            Token::Literal(&[0x01]),
                        ],
                    },
                },
            ],
        },
        Signature {
            byte_sequences: &[
                ByteSequence {
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
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(32_768),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x43, 0x44, 0x30, 0x30, 0x31]),
                            Token::WildcardCountRange(1, 16_384),
                            Token::Literal(&[0xFF, 0x43, 0x44, 0x30, 0x30, 0x31]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(32_768),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00, 0x42, 0x45, 0x41, 0x30, 0x31, 0x01]),
                            Token::WildcardCount(2_041),
                            Token::Literal(&[0x00, 0x4E, 0x53, 0x52, 0x30]),
                            Token::Any(&[&[Token::Literal(&[0x32])], &[Token::Literal(&[0x33])]]),
                            Token::Literal(&[0x01]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_584,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_585,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_586,
        },
    ],
};
