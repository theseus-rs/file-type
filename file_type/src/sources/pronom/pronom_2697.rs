use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2697: FileFormat = FileFormat {
    id: 2_697,
    source_type: SourceType::Pronom,
    name: "Final Draft Document",
    extensions: &["fdx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x20, 0x73, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x6C, 0x6F, 0x6E, 0x65, 0x3D,
                        0x22, 0x6E, 0x6F, 0x22, 0x20, 0x3F, 0x3E, 0x0A, 0x3C, 0x46, 0x69, 0x6E,
                        0x61, 0x6C, 0x44, 0x72, 0x61, 0x66, 0x74, 0x20, 0x44, 0x6F, 0x63, 0x75,
                        0x6D, 0x65, 0x6E, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3D,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x2F, 0x46, 0x69, 0x6E, 0x61, 0x6C, 0x44, 0x72, 0x61, 0x66, 0x74,
                        0x3E, 0x0A,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_769,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 638,
        },
    ],
};
