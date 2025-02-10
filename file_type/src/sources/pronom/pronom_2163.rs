use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2163: FileType = FileType {
    file_format: &FileFormat {
        id: 2_163,
        source_type: SourceType::Pronom,
        name: "Legacy Family Tree Database",
        extensions: &["fdb"],
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
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4C, 0x65, 0x67, 0x61, 0x63, 0x79, 0x31])],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 352,
        }],
    },
};
