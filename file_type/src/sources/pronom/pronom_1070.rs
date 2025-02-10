use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1070: FileType = FileType {
    file_format: &FileFormat {
        id: 1_070,
        source_type: SourceType::Pronom,
        name: "EndNote Library",
        extensions: &["enl"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x03])]]),
                            Token::WildcardCount(2),
                            Token::Range(&[0x00], &[0x0F]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0xFE]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3F]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
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
                            Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x03])]]),
                            Token::WildcardCount(2),
                            Token::Range(&[0x00], &[0x0F]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0xFF]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3F]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00]),
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
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(7),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x00, 0x02]),
                            Token::WildcardCount(8),
                            Token::Literal(&[
                                0x20, 0x20, 0x40, 0x40, 0x40, 0x20, 0x00, 0x00, 0x40, 0x40, 0x40,
                                0x40,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_518,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_519,
            },
        ],
    },
};
