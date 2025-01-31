use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_413: FileFormat = FileFormat {
    id: 1_191,
    puid: "fmt/413",
    name: "Scalable Vector Graphics Tiny",
    extensions: &["svg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x3C, 0x73, 0x76, 0x67]),
                    Token::WildcardCountRange(0, 512),
                    Token::Literal(&[
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x32,
                        0x22,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x73, 0x76, 0x67, 0x3E]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 634,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 634,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 635,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
