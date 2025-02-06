use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1670: FileFormat = FileFormat {
    id: 1_670,
    source_type: SourceType::Pronom,
    name: "Apple Safari Webarchive",
    extensions: &["webarchive"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30]),
                    Token::WildcardCountRange(6, 500),
                    Token::Literal(&[
                        0x57, 0x65, 0x62, 0x4D, 0x61, 0x69, 0x6E, 0x52, 0x65, 0x73, 0x6F, 0x75,
                        0x72, 0x63, 0x65,
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
            id: 1_258,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_789,
        },
    ],
};
