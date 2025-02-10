use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1018: FileType = FileType {
    file_format: &FileFormat {
        id: 1_018,
        source_type: SourceType::Pronom,
        name: "Internet Message Format",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x0D, 0x0A, 0x58, 0x2D, 0x4D, 0x69, 0x6D, 0x65, 0x4F, 0x4C, 0x45,
                                0x3A, 0x20, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x65, 0x64, 0x20,
                                0x42, 0x79, 0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66,
                                0x74, 0x20, 0x4D, 0x69, 0x6D, 0x65, 0x4F, 0x4C, 0x45, 0x20, 0x56,
                                0x36, 0x2E, 0x30, 0x30, 0x2E,
                            ]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x2E]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x0D, 0x0A]),
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
                            Token::Literal(&[
                                0x0D, 0x0A, 0x58, 0x2D, 0x43, 0x6F, 0x6E, 0x76, 0x65, 0x72, 0x74,
                                0x65, 0x64, 0x2D, 0x42, 0x79, 0x3A, 0x20, 0x45, 0x6D, 0x61, 0x69,
                                0x6C, 0x63, 0x68, 0x65, 0x6D, 0x79, 0x20,
                            ]),
                            Token::SingleWildcard,
                            Token::Literal(&[0x2E]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 641,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 820,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_755,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 645,
            },
        ],
    },
};
