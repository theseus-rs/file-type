use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2029: FileType = FileType {
    file_format: &FileFormat {
        id: 2_029,
        source_type: SourceType::Pronom,
        name: "Gnumeric",
        extensions: &["gnumeric"],
        media_types: &["application/x-gnumeric"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                                0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(2, 32),
                            Token::Literal(&[
                                0x3C, 0x67, 0x6E, 0x6D, 0x3A, 0x57, 0x6F, 0x72, 0x6B, 0x62, 0x6F,
                                0x6F, 0x6B, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x67, 0x6E,
                                0x6D, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77,
                                0x77, 0x77, 0x2E, 0x67, 0x6E, 0x75, 0x6D, 0x65, 0x72, 0x69, 0x63,
                                0x2E, 0x6F, 0x72, 0x67,
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
                                0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30,
                            ]),
                            Token::WildcardCountRange(2, 32),
                            Token::Literal(&[
                                0x3C, 0x67, 0x6D, 0x72, 0x3A, 0x57, 0x6F, 0x72, 0x6B, 0x62, 0x6F,
                                0x6F, 0x6B, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x67, 0x6D,
                                0x72, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77,
                                0x77, 0x77, 0x2E, 0x67, 0x6E, 0x6F, 0x6D, 0x65, 0x2E, 0x6F, 0x72,
                                0x67, 0x2F, 0x67, 0x6E, 0x75, 0x6D, 0x65, 0x72, 0x69, 0x63, 0x2F,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanBeContainedBy,
                id: 386,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 638,
            },
        ],
    },
};
