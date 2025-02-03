use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1868: FileFormat = FileFormat {
    id: 1_868,
    source_type: SourceType::Pronom,
    name: "Hasselblad 3FR Raw Image",
    extensions: &["3fr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCountRange(4, 128),
                        Token::Literal(&[0x0F, 0x01]),
                        Token::WildcardCountRange(8, 512),
                        Token::Literal(&[
                            0x48, 0x61, 0x73, 0x73, 0x65, 0x6C, 0x62, 0x6C, 0x61, 0x64,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0x00, 0x04, 0x00, 0x01])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 672,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 673,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 752,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        },
    ],
};
