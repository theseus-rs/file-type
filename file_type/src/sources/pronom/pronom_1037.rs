use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1037: FileType = FileType {
    file_format: &FileFormat {
        id: 1_037,
        source_type: SourceType::Pronom,
        name: "OpenDocument Spreadsheet",
        extensions: &["ods", "ots"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::WildcardCount(26),
                            Token::Literal(&[
                                0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70,
                                0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E,
                                0x64, 0x2E, 0x6F, 0x61, 0x73, 0x69, 0x73, 0x2E, 0x6F, 0x70, 0x65,
                                0x6E, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x2E, 0x73,
                                0x70, 0x72, 0x65, 0x61, 0x64, 0x73, 0x68, 0x65, 0x65, 0x74, 0x50,
                                0x4B, 0x03, 0x04, 0x14,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x76, 0x65, 0x72, 0x73,
                                0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x31, 0x22,
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
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::WildcardCount(26),
                            Token::Literal(&[
                                0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70,
                                0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E,
                                0x64, 0x2E, 0x6F, 0x61, 0x73, 0x69, 0x73, 0x2E, 0x6F, 0x70, 0x65,
                                0x6E, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x2E, 0x73,
                                0x70, 0x72, 0x65, 0x61, 0x64, 0x73, 0x68, 0x65, 0x65, 0x74, 0x50,
                                0x4B, 0x03, 0x04, 0x14,
                            ]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_038,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_603,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
        ],
    },
};
