use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_783: FileFormat = FileFormat {
    id: 783,
    source_type: SourceType::Pronom,
    name: "OpenDocument Database Format",
    extensions: &["odb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                    Token::WildcardCount(26),
                    Token::Literal(&[
                        0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70, 0x6C,
                        0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E, 0x64, 0x2E,
                        0x73, 0x75, 0x6E, 0x2E, 0x78, 0x6D, 0x6C, 0x2E, 0x62, 0x61, 0x73, 0x65,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 777,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 778,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_231,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 778,
        },
    ],
};
