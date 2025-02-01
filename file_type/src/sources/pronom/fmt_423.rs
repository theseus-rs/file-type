use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_423: FileFormat = FileFormat {
    id: 1_205,
    puid: "fmt/423",
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
                    Token::WildcardCountRange(128, 1_024),
                    Token::Literal(&[
                        0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F, 0x72,
                        0x6D, 0x61, 0x74, 0x20, 0x33,
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
    ],
};
