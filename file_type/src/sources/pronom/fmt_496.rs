use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_496: FileFormat = FileFormat {
    id: 1_283,
    puid: "fmt/496",
    name: "TransXchange File Format",
    extensions: &["txc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x31, 0x2E, 0x30]),
                    Token::WildcardCountRange(0, 64),
                    Token::Literal(&[
                        0x3C, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x58, 0x43, 0x68, 0x61, 0x6E, 0x67,
                        0x65, 0x20, 0x78, 0x73, 0x69, 0x3A, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61,
                        0x4C, 0x6F, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x68, 0x74,
                        0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x74, 0x72, 0x61,
                        0x6E, 0x73, 0x78, 0x63, 0x68, 0x61, 0x6E, 0x67, 0x65, 0x2E, 0x6F, 0x72,
                        0x67, 0x2E, 0x75, 0x6B, 0x2F,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
