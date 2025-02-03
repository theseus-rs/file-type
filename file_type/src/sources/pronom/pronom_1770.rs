use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1770: FileFormat = FileFormat {
    id: 1_770,
    source_type: SourceType::Pronom,
    name: "Music Encoding Initiative",
    extensions: &["mei"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
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
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(20),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                        0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6D, 0x75, 0x73, 0x69, 0x63, 0x2D,
                        0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x2E, 0x6F, 0x72, 0x67,
                        0x2F, 0x6E, 0x73, 0x2F, 0x6D, 0x65, 0x69,
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
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 638,
        },
    ],
};
