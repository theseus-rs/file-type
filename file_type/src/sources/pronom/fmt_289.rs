use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_289: FileFormat = FileFormat {
    id: 1_029,
    puid: "fmt/289",
    name: "WARC",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x41, 0x52, 0x43, 0x2F]),
                    Token::WildcardCountRange(0, 517),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x57, 0x41, 0x52, 0x43, 0x2D, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x2D,
                            0x49, 0x44,
                        ])],
                        &[Token::Literal(&[
                            0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x4C, 0x65, 0x6E, 0x67,
                            0x74, 0x68,
                        ])],
                        &[Token::Literal(&[
                            0x57, 0x41, 0x52, 0x43, 0x2D, 0x54, 0x79, 0x70, 0x65,
                        ])],
                        &[Token::Literal(&[
                            0x57, 0x41, 0x52, 0x43, 0x2D, 0x44, 0x61, 0x74, 0x65,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_099,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_173,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 639,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 640,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 641,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 642,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 643,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 644,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 645,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_258,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
