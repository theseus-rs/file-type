use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_669: FileType = FileType {
    file_format: &FileFormat {
        id: 669,
        source_type: SourceType::Pronom,
        name: "JPEG File Interchange Format",
        extensions: &["jpg", "jpe", "jpeg", "jif", "jfif", "jfi"],
        media_types: &["image/jpeg"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0xFF, 0xD8, 0xFF, 0xE0]),
                            Token::WildcardCount(2),
                            Token::Literal(&[0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02]),
                            Token::Any(&[
                                &[Token::Literal(&[0x00])],
                                &[Token::Literal(&[0x01])],
                                &[Token::Literal(&[0x02])],
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xD9])],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 670,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 668,
            },
        ],
    },
};
