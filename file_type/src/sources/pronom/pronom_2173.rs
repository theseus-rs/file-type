use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2173: FileFormat = FileFormat {
    id: 2_173,
    source_type: SourceType::Pronom,
    name: "WARC",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x41, 0x52, 0x43, 0x2F, 0x31, 0x2E, 0x30, 0x0D, 0x0A]),
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
            relationship_type: RelationshipType::HasPriorityOver,
            id: 639,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 640,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 641,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 642,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 643,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 644,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 645,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_029,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_258,
        },
    ],
};
