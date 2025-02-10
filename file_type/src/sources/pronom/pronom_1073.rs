use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1073: FileType = FileType {
    file_format: &FileFormat {
        id: 1_073,
        source_type: SourceType::Pronom,
        name: "EndNote Import File",
        extensions: &["enw", "enr"],
        media_types: &["application/x-endnote-refer"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x25, 0x41, 0x20]),
                            Token::Range(&[0x41], &[0x5A]),
                            Token::WildcardCountRange(5, 50),
                            Token::Any(&[
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0A, 0x0D])],
                            ]),
                            Token::Literal(&[0x25]),
                            Token::Any(&[
                                &[Token::Literal(&[0x41])],
                                &[Token::Literal(&[0x42])],
                                &[Token::Literal(&[0x43])],
                                &[Token::Literal(&[0x44])],
                                &[Token::Literal(&[0x54])],
                            ]),
                            Token::Literal(&[0x20]),
                            Token::Any(&[
                                &[Token::Range(&[0x30], &[0x39])],
                                &[Token::Range(&[0x41], &[0x5A])],
                            ]),
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
                            Token::Literal(&[0x25, 0x41]),
                            Token::Range(&[0x41], &[0x5A]),
                            Token::WildcardCountRange(5, 50),
                            Token::Any(&[
                                &[Token::Literal(&[0x0A])],
                                &[Token::Literal(&[0x0A, 0x0D])],
                            ]),
                            Token::Literal(&[0x25]),
                            Token::Any(&[
                                &[Token::Literal(&[0x41])],
                                &[Token::Literal(&[0x42])],
                                &[Token::Literal(&[0x43])],
                                &[Token::Literal(&[0x44])],
                                &[Token::Literal(&[0x54])],
                            ]),
                            Token::Any(&[
                                &[Token::Range(&[0x30], &[0x39])],
                                &[Token::Range(&[0x41], &[0x5A])],
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 86,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 138,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 331,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 332,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 771,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 772,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 773,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_288,
            },
        ],
    },
};
