use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2502: FileFormat = FileFormat {
    id: 2_502,
    source_type: SourceType::Pronom,
    name: "Roxio Easy Media Creator Layout",
    extensions: &["rcl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0xFE, 0x23, 0x00]),
                    Token::WildcardCountRange(0, 900),
                    Token::Literal(&[
                        0x5B, 0x00, 0x52, 0x00, 0x6F, 0x00, 0x78, 0x00, 0x69, 0x00, 0x6F, 0x00,
                        0x49, 0x00, 0x4E, 0x00, 0x46, 0x00, 0x32, 0x00, 0x30, 0x00, 0x5D, 0x00,
                        0x0D, 0x00, 0x0A, 0x00, 0x5B, 0x00, 0x50, 0x00, 0x72, 0x00, 0x6F, 0x00,
                        0x6A, 0x00, 0x65, 0x00, 0x63, 0x00, 0x74, 0x00, 0x53, 0x00, 0x65, 0x00,
                        0x74, 0x00, 0x74, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x73, 0x00,
                        0x5D, 0x00, 0x0D, 0x00, 0x0A, 0x00, 0x52, 0x00, 0x43, 0x00, 0x4C, 0x00,
                        0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x3D, 0x00, 0x37, 0x00, 0x30,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_503,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_504,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_505,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_501,
        },
    ],
};
