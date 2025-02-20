use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1029: FileType = FileType {
    file_format: &FileFormat {
        id: 1_029,
        source_type: SourceType::Pronom,
        name: "WARC",
        extensions: &["warc"],
        media_types: &["application/warc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x57, 0x41, 0x52, 0x43, 0x2F]),
                        Token::WildcardCountRange(0, 517),
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x57, 0x41, 0x52, 0x43, 0x2D, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64,
                                0x2D, 0x49, 0x44,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x4C, 0x65, 0x6E,
                                0x67, 0x74, 0x68,
                            ])],
                            &[Token::Literal(&[
                                0x57, 0x41, 0x52, 0x43, 0x2D, 0x54, 0x79, 0x70, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x57, 0x41, 0x52, 0x43, 0x2D, 0x44, 0x61, 0x74, 0x65,
                            ])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_099,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_173,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 639,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 640,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 641,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 642,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 643,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 644,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 645,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_258,
            },
        ],
    },
};
