use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1755: FileType = FileType {
    file_format: &FileFormat {
        id: 1_755,
        source_type: SourceType::Pronom,
        name: "MIME Email",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D]),
                            Token::Any(&[
                                &[Token::Literal(&[0x49, 0x4D, 0x45])],
                                &[Token::Literal(&[0x69, 0x6D, 0x65])],
                            ]),
                            Token::Literal(&[0x2D]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Literal(&[
                                0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x30,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x54, 0x6F, 0x3A, 0x20])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x3A, 0x20])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x44, 0x61, 0x74, 0x65, 0x3A, 0x20])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D]),
                            Token::Any(&[&[Token::Literal(&[0x54])], &[Token::Literal(&[0x74])]]),
                            Token::Literal(&[0x79, 0x70, 0x65, 0x3A, 0x20]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_930,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_018,
            },
        ],
    },
};
