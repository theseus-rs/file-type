use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_562: FileFormat = FileFormat {
    id: 1_350,
    puid: "fmt/562",
    name: "Adobe Illustrator",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x35]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x3C, 0x69, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72, 0x61, 0x74, 0x6F, 0x72,
                        0x3A, 0x54, 0x79, 0x70, 0x65, 0x3E, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65,
                        0x6E, 0x74, 0x3C, 0x2F, 0x69, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72, 0x61,
                        0x74, 0x6F, 0x72, 0x3A, 0x54, 0x79, 0x70, 0x65, 0x3E,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                        0x2E, 0x30,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F, 0x72,
                        0x6D, 0x61, 0x74, 0x20, 0x39,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 618,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_351,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_349,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
