use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_866: FileFormat = FileFormat {
    id: 1_670,
    puid: "fmt/866",
    name: "Apple Safari Webarchive",
    extensions: &["webarchive"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        RelatedFormat {
            id: 1_789,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
