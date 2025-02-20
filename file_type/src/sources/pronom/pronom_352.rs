use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_352: FileType = FileType {
    file_format: &FileFormat {
        id: 352,
        source_type: SourceType::Pronom,
        name: "Microsoft Access Database File",
        extensions: &["mdb", "mde"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64,
                            0x20, 0x4A, 0x65, 0x74, 0x20, 0x44, 0x42, 0x00, 0x01, 0x00, 0x00, 0x00,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x41, 0x00, 0x63, 0x00, 0x63, 0x00, 0x65, 0x00, 0x73, 0x00, 0x73,
                                0x00, 0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
                                0x6F, 0x00, 0x6E,
                            ]),
                            Token::WildcardCountRange(0, 2_048),
                            Token::Literal(&[0x30, 0x00, 0x38, 0x00, 0x2E, 0x00]),
                            Token::Range(&[0x30], &[0x39]),
                            Token::Literal(&[0x00]),
                            Token::Range(&[0x30], &[0x39]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_163,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 353,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 351,
            },
        ],
    },
};
