use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_350: FileType = FileType {
    file_format: &FileFormat {
        id: 350,
        source_type: SourceType::Pronom,
        name: "Microsoft Access Database File",
        extensions: &["mdb", "mda", "mde", "mdt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64,
                            0x20, 0x4A, 0x65, 0x74, 0x20, 0x44, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x56, 0x65, 0x72, 0x73, 0x69,
                                0x6F, 0x6E,
                            ]),
                            Token::WildcardCountRange(0, 1_024),
                            Token::Literal(&[0x30, 0x36, 0x2E]),
                            Token::Range(&[0x30], &[0x39]),
                            Token::Range(&[0x30], &[0x39]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 351,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 107,
            },
        ],
    },
};
