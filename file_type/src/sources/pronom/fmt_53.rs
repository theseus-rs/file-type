use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_53: FileFormat = FileFormat {
    id: 753,
    puid: "fmt/53",
    name: "Rich Text Format",
    extensions: &["rtf"],
    media_types: &["application/rtf", "text/rtf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x7B, 0x5C, 0x72, 0x74]),
                    Token::Any(&[&[Token::Literal(&[0x66])], &[Token::Literal(&[0x66, 0x31])]]),
                    Token::WildcardCountRange(0, 15),
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
                    Token::AnyWildcard,
                    Token::Literal(&[0x5C, 0x6C, 0x73, 0x64, 0x73, 0x74, 0x69, 0x6D, 0x61, 0x78]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_101,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 626,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 627,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 628,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 629,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 630,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 631,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 632,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 633,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_101,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 633,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
