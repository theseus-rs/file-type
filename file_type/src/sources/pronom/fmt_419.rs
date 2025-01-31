use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_419: FileFormat = FileFormat {
    id: 1_201,
    puid: "fmt/419",
    name: "Adobe Illustrator",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                        0x2E, 0x30,
                    ]),
                    Token::WildcardCountRange(1, 128),
                    Token::Literal(&[
                        0x25, 0x25, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x3A, 0x20, 0x41,
                        0x64, 0x6F, 0x62, 0x65, 0x20, 0x49, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72,
                        0x61, 0x74, 0x6F, 0x72, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x66, 0x6F, 0x72,
                        0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x2C, 0x20, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34, 0x2E, 0x30,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 86,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 331,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 332,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 771,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 773,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_200,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
