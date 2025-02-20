use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2586: FileType = FileType {
    file_format: &FileFormat {
        id: 2_586,
        source_type: SourceType::Pronom,
        name: "Apple Partition Map Disk Image",
        extensions: &["toast", "iso", "cdr", "dmg", "bin", "img"],
        media_types: &[],
        signatures: &[
            Signature {
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
            Signature {
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
            Signature {
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
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_878,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_587,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_605,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_255,
            },
        ],
    },
};
