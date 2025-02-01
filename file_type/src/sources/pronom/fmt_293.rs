use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_293: FileFormat = FileFormat {
    id: 1_036,
    puid: "fmt/293",
    name: "OpenDocument Presentation",
    extensions: &["odp", "otp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
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
                        0x6F, 0x61, 0x73, 0x69, 0x73, 0x2E, 0x6F, 0x70, 0x65, 0x6E, 0x64, 0x6F,
                        0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x2E, 0x70, 0x72, 0x65, 0x73, 0x65,
                        0x6E, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x32, 0x22,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_035,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
