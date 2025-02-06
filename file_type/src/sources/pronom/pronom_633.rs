use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_633: FileFormat = FileFormat {
    id: 633,
    source_type: SourceType::Pronom,
    name: "Rich Text Format",
    extensions: &["rtf"],
    media_types: &["application/rtf", "text/rtf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x7B, 0x5C, 0x72, 0x74]),
                    Token::Any(&[&[Token::Literal(&[0x66])], &[Token::Literal(&[0x66, 0x31])]]),
                    Token::Literal(&[0x5C]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61, 0x6E, 0x73, 0x69])],
                        &[Token::Literal(&[0x6D, 0x61, 0x63])],
                        &[Token::Literal(&[0x70, 0x63])],
                        &[Token::Literal(&[0x70, 0x63, 0x61])],
                    ]),
                    Token::Literal(&[0x5C, 0x61, 0x6E, 0x73, 0x69, 0x63, 0x70, 0x67]),
                    Token::WildcardCountRange(3, 4_294_967_295),
                    Token::Literal(&[0x5C, 0x73, 0x74, 0x73, 0x68, 0x66, 0x64, 0x62, 0x63, 0x68]),
                    Token::WildcardCountRange(1, 4),
                    Token::Literal(&[0x5C, 0x73, 0x74, 0x73, 0x68, 0x66, 0x6C, 0x6F, 0x63, 0x68]),
                    Token::WildcardCountRange(1, 4),
                    Token::Literal(&[0x5C, 0x73, 0x74, 0x73, 0x68, 0x66, 0x68, 0x69, 0x63, 0x68]),
                    Token::WildcardCountRange(1, 4),
                    Token::Literal(&[0x5C, 0x73, 0x74, 0x73, 0x68, 0x66, 0x62, 0x69]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 753,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_101,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 626,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 627,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 628,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 629,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 630,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 631,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 632,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 753,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 632,
        },
    ],
};
