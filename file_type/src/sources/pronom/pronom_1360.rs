use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1360: FileType = FileType {
    file_format: &FileFormat {
        id: 1_360,
        source_type: SourceType::Pronom,
        name: "Domino XML Database Export",
        extensions: &["dxl"],
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
                            Token::WildcardCountRange(1, 64),
                            Token::Literal(&[
                                0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x64,
                                0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                            ]),
                            Token::WildcardCountRange(1, 64),
                            Token::Literal(&[
                                0x3C, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x78,
                                0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                                0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6C, 0x6F, 0x74, 0x75, 0x73,
                                0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x64, 0x78, 0x6C, 0x22,
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
                            Token::Literal(&[
                                0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                                0x6F, 0x6E, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x31, 0x2E, 0x30]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[0x20]),
                            Token::WildcardCountRange(1, 64),
                            Token::Literal(&[
                                0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x78,
                                0x6D, 0x6C, 0x6E, 0x73, 0x3D,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                            Token::Literal(&[
                                0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E,
                                0x6C, 0x6F, 0x74, 0x75, 0x73, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x64,
                                0x78, 0x6C,
                            ]),
                            Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
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
