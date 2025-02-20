use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_506: FileType = FileType {
    file_format: &FileFormat {
        id: 506,
        source_type: SourceType::Pronom,
        name: "Microsoft FoxPro Memo",
        extensions: &["fpt", "frt", "vct", "pjt"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(504),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::Range(&[0x00], &[0x02]),
                            Token::WildcardCount(4),
                            Token::NotLiteral(&[0x00]),
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
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(520),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::Range(&[0x00], &[0x02]),
                            Token::WildcardCount(4),
                            Token::NotLiteral(&[0x00]),
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
                            Token::Literal(&[0x00]),
                            Token::WildcardCount(3),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::NotLiteral(&[0x00]),
                            Token::WildcardCount(504),
                            Token::Literal(&[0x00, 0x00, 0x00]),
                            Token::Range(&[0x00], &[0x02]),
                            Token::Literal(&[
                                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::NotLiteral(&[0x00]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_131,
        }],
    },
};
