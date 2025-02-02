use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1045: FileFormat = FileFormat {
    id: 1_045,
    source_type: SourceType::Pronom,
    name: "Computer Graphics Metafile ASCII",
    extensions: &["cgm"],
    media_types: &["image/cgm"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x42]),
                            Token::Any(&[&[Token::Literal(&[0x45])], &[Token::Literal(&[0x65])]]),
                            Token::Any(&[&[Token::Literal(&[0x47])], &[Token::Literal(&[0x67])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Any(&[&[Token::Literal(&[0x65])], &[Token::Literal(&[0x45])]]),
                            Token::Any(&[&[Token::Literal(&[0x52])], &[Token::Literal(&[0x72])]]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Any(&[&[Token::Literal(&[0x49])], &[Token::Literal(&[0x69])]]),
                            Token::Any(&[&[Token::Literal(&[0x4F])], &[Token::Literal(&[0x6F])]]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x33]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x45]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::Any(&[&[Token::Literal(&[0x44])], &[Token::Literal(&[0x64])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3B]),
                        ],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x42]),
                            Token::Any(&[&[Token::Literal(&[0x45])], &[Token::Literal(&[0x65])]]),
                            Token::Any(&[&[Token::Literal(&[0x47])], &[Token::Literal(&[0x67])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Any(&[&[Token::Literal(&[0x65])], &[Token::Literal(&[0x45])]]),
                            Token::Any(&[&[Token::Literal(&[0x52])], &[Token::Literal(&[0x72])]]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Any(&[&[Token::Literal(&[0x49])], &[Token::Literal(&[0x69])]]),
                            Token::Any(&[&[Token::Literal(&[0x4F])], &[Token::Literal(&[0x6F])]]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x33]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x65]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::Any(&[&[Token::Literal(&[0x44])], &[Token::Literal(&[0x64])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3B]),
                        ],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x62]),
                            Token::Any(&[&[Token::Literal(&[0x45])], &[Token::Literal(&[0x65])]]),
                            Token::Any(&[&[Token::Literal(&[0x47])], &[Token::Literal(&[0x67])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Any(&[&[Token::Literal(&[0x65])], &[Token::Literal(&[0x45])]]),
                            Token::Any(&[&[Token::Literal(&[0x52])], &[Token::Literal(&[0x72])]]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Any(&[&[Token::Literal(&[0x49])], &[Token::Literal(&[0x69])]]),
                            Token::Any(&[&[Token::Literal(&[0x4F])], &[Token::Literal(&[0x6F])]]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x33]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x45]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::Any(&[&[Token::Literal(&[0x44])], &[Token::Literal(&[0x64])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3B]),
                        ],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x62]),
                            Token::Any(&[&[Token::Literal(&[0x45])], &[Token::Literal(&[0x65])]]),
                            Token::Any(&[&[Token::Literal(&[0x47])], &[Token::Literal(&[0x67])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                            Token::Any(&[&[Token::Literal(&[0x65])], &[Token::Literal(&[0x45])]]),
                            Token::Any(&[&[Token::Literal(&[0x52])], &[Token::Literal(&[0x72])]]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Any(&[&[Token::Literal(&[0x49])], &[Token::Literal(&[0x69])]]),
                            Token::Any(&[&[Token::Literal(&[0x4F])], &[Token::Literal(&[0x6F])]]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x33]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x65]),
                            Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x6E])]]),
                            Token::Any(&[&[Token::Literal(&[0x44])], &[Token::Literal(&[0x64])]]),
                            Token::Any(&[&[Token::Literal(&[0x4D])], &[Token::Literal(&[0x6D])]]),
                            Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x66])]]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x3B]),
                        ],
                    },
                },
            ],
        },
    ],
    related_formats: &[],
};
