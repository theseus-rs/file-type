use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1067: FileFormat = FileFormat {
    id: 1_874,
    puid: "fmt/1067",
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x21, 0x3C, 0x3E, 0x3C, 0x50, 0x44, 0x42, 0x3E, 0x3C, 0x3E, 0x21,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x5F, 0x73, 0x69, 0x6C, 0x6F, 0x6C, 0x69, 0x62, 0x69, 0x6E, 0x66, 0x6F,
                        ])],
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
                        tokens: &[Token::Literal(&[
                            0x21, 0x3C, 0x3C, 0x50, 0x44, 0x42, 0x3A, 0x49, 0x49, 0x3E, 0x3E, 0x21,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x5F, 0x73, 0x69, 0x6C, 0x6F, 0x6C, 0x69, 0x62, 0x69, 0x6E, 0x66, 0x6F,
                        ])],
                    },
                },
            ],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_871,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_872,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
