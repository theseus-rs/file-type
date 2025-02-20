use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2702: FileType = FileType {
    file_format: &FileFormat {
        id: 2_702,
        source_type: SourceType::Pronom,
        name: "WordPerfect Macro File",
        extensions: &["wpm", "wcm"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x01, 0x01]),
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
                            Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x01, 0x0A]),
                            Token::WildcardCountRange(0, 4_000),
                            Token::Any(&[
                                &[Token::Literal(&[0x77, 0x63, 0x6D])],
                                &[Token::Literal(&[0x57, 0x43, 0x4D])],
                            ]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x61])], &[Token::Literal(&[0x41])]]),
                            Token::Literal(&[
                                0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                            ]),
                            Token::WildcardCountRange(0, 15),
                            Token::Any(&[&[Token::Literal(&[0x64])], &[Token::Literal(&[0x44])]]),
                            Token::Literal(&[0x65, 0x66, 0x61, 0x75, 0x6C, 0x74]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 736,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 737,
            },
        ],
    },
};
