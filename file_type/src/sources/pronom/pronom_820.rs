use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_820: FileFormat = FileFormat {
    id: 820,
    source_type: SourceType::Pronom,
    name: "MHTML",
    extensions: &["mht", "mhtml"],
    media_types: &["multipart/related"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D]),
                    Token::Any(&[
                        &[Token::Literal(&[0x49, 0x4D, 0x45])],
                        &[Token::Literal(&[0x69, 0x6D, 0x65])],
                    ]),
                    Token::Literal(&[0x2D]),
                    Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                    Token::Literal(&[
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x30,
                    ]),
                    Token::WildcardCountRange(1, 2),
                    Token::Literal(&[
                        0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x54, 0x79, 0x70, 0x65,
                        0x3A, 0x20, 0x6D, 0x75, 0x6C, 0x74, 0x69, 0x70, 0x61, 0x72, 0x74, 0x2F,
                        0x72, 0x65, 0x6C, 0x61, 0x74, 0x65, 0x64,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
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
            id: 1_018,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_258,
        },
    ],
};
