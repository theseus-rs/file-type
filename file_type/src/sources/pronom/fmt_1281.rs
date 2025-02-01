use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1281: FileFormat = FileFormat {
    id: 2_099,
    puid: "fmt/1281",
    name: "WARC",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x41, 0x52, 0x43, 0x2F, 0x31, 0x2E, 0x31, 0x0D, 0x0A]),
                    Token::WildcardCountRange(0, 512),
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
            id: 1_029,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_258,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
