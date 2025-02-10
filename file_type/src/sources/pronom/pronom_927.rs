use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_927: FileType = FileType {
    file_format: &FileFormat {
        id: 927,
        source_type: SourceType::Pronom,
        name: "Nikon Digital SLR Camera Raw Image File",
        extensions: &["nef", "nrw"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x92, 0x7C]),
                            Token::WildcardCountRange(0, 35_536),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08,
                            ]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x01, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01]),
                            Token::Any(&[
                                &[Token::Literal(&[0x00, 0x01])],
                                &[Token::Literal(&[0x87, 0x99])],
                            ]),
                            Token::Literal(&[0x00, 0x00]),
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
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x7C, 0x92]),
                            Token::WildcardCountRange(0, 35_536),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00,
                            ]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x00, 0x03, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x99, 0x87])],
                            ]),
                            Token::Literal(&[0x00]),
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
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x92, 0x7C]),
                            Token::WildcardCountRange(0, 35_536),
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x01, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01]),
                            Token::Any(&[
                                &[Token::Literal(&[0x00, 0x01])],
                                &[Token::Literal(&[0x87, 0x99])],
                            ]),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08,
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
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x7C, 0x92]),
                            Token::WildcardCountRange(0, 35_536),
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x00, 0x03, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x99, 0x87])],
                            ]),
                            Token::Literal(&[0x00]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00,
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
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x01, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01]),
                            Token::Any(&[
                                &[Token::Literal(&[0x00, 0x01])],
                                &[Token::Literal(&[0x87, 0x99])],
                            ]),
                            Token::Literal(&[0x00, 0x00]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[0x92, 0x7C]),
                            Token::WildcardCountRange(0, 2_046),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08,
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
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x00, 0xFE, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x00,
                            ]),
                            Token::WildcardCount(36),
                            Token::Literal(&[0x00, 0x03, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x01, 0x00])],
                                &[Token::Literal(&[0x99, 0x87])],
                            ]),
                            Token::Literal(&[0x00]),
                            Token::WildcardCountRange(0, 999_999),
                            Token::Literal(&[0x7C, 0x92]),
                            Token::WildcardCountRange(0, 2_046),
                            Token::Literal(&[0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00]),
                            Token::Any(&[
                                &[Token::Literal(&[0x02, 0x00])],
                                &[Token::Literal(&[0x02, 0x10])],
                                &[Token::Literal(&[0x02, 0x11])],
                            ]),
                            Token::Literal(&[
                                0x00, 0x00, 0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00,
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
                            Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x4E, 0x49, 0x4B, 0x4F, 0x4E, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F,
                                0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x00, 0x4E, 0x49, 0x4B, 0x4F,
                                0x4E, 0x20, 0x44, 0x31, 0x20, 0x00,
                            ]),
                            Token::WildcardCountRange(1, 16_384),
                            Token::Literal(&[0x92, 0x7C]),
                            Token::WildcardCountRange(0, 512),
                            Token::Literal(&[0x52, 0x41, 0x57]),
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
                            Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x4E, 0x49, 0x4B, 0x4F, 0x4E, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F,
                                0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x00, 0x4E, 0x49, 0x4B, 0x4F,
                                0x4E, 0x20, 0x44, 0x31, 0x20, 0x00,
                            ]),
                            Token::WildcardCountRange(1, 16_384),
                            Token::Literal(&[0x7C, 0x92]),
                            Token::WildcardCountRange(0, 512),
                            Token::Literal(&[0x52, 0x41, 0x57]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 795,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_223,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_224,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_225,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_529,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_693,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_694,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 797,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_099,
            },
        ],
    },
};
