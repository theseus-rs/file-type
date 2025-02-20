use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1452: FileType = FileType {
    file_format: &FileFormat {
        id: 1_452,
        source_type: SourceType::Pronom,
        name: "INTERLIS Transfer File",
        extensions: &["xtf"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                                0x6F, 0x6E, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x31, 0x2E, 0x30]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x55, 0x54, 0x46, 0x2D, 0x38]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::WildcardCountRange(0, 2),
                            Token::Literal(&[0x3F, 0x3E]),
                            Token::WildcardCountRange(0, 256),
                            Token::Literal(&[
                                0x3C, 0x54, 0x52, 0x41, 0x4E, 0x53, 0x46, 0x45, 0x52, 0x20, 0x78,
                                0x6D, 0x6C, 0x6E, 0x73, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E,
                                0x69, 0x6E, 0x74, 0x65, 0x72, 0x6C, 0x69, 0x73, 0x2E, 0x63, 0x68,
                                0x2F, 0x49, 0x4E, 0x54, 0x45, 0x52, 0x4C, 0x49, 0x53, 0x32, 0x2E,
                                0x33,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
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
                                0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00, 0x20,
                                0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
                                0x6F, 0x00, 0x6E, 0x00, 0x3D, 0x00,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x00, 0x31, 0x00, 0x2E, 0x00, 0x30, 0x00]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x00, 0x20, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x63, 0x00, 0x6F, 0x00,
                                0x64, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x3D, 0x00,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x00, 0x55, 0x00, 0x54, 0x00, 0x46, 0x00, 0x2D, 0x00, 0x38, 0x00,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x00, 0x3F, 0x00, 0x3E]),
                            Token::WildcardCountRange(0, 512),
                            Token::Literal(&[
                                0x3C, 0x00, 0x54, 0x00, 0x52, 0x00, 0x41, 0x00, 0x4E, 0x00, 0x53,
                                0x00, 0x46, 0x00, 0x45, 0x00, 0x52, 0x00, 0x20, 0x00, 0x78, 0x00,
                                0x6D, 0x00, 0x6C, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x3D, 0x00,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x00, 0x68, 0x00, 0x74, 0x00, 0x74, 0x00, 0x70, 0x00, 0x3A, 0x00,
                                0x2F, 0x00, 0x2F, 0x00, 0x77, 0x00, 0x77, 0x00, 0x77, 0x00, 0x2E,
                                0x00, 0x69, 0x00, 0x6E, 0x00, 0x74, 0x00, 0x65, 0x00, 0x72, 0x00,
                                0x6C, 0x00, 0x69, 0x00, 0x73, 0x00, 0x2E, 0x00, 0x63, 0x00, 0x68,
                                0x00, 0x2F, 0x00, 0x49, 0x00, 0x4E, 0x00, 0x54, 0x00, 0x45, 0x00,
                                0x52, 0x00, 0x4C, 0x00, 0x49, 0x00, 0x53, 0x00, 0x32, 0x00, 0x2E,
                                0x00, 0x33,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
