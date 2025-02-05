use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2766: FileFormat = FileFormat {
    id: 2_766,
    source_type: SourceType::Pronom,
    name: "Acrobat PDF/A - Portable Document Format",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x32, 0x2E]),
                        Token::Range(&[0x30], &[0x39]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61, 0x72, 0x74,
                        0x3E, 0x34, 0x3C, 0x2F, 0x70, 0x64, 0x66, 0x61, 0x69, 0x64, 0x3A, 0x70,
                        0x61, 0x72, 0x74, 0x3E,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_767,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_770,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_939,
        },
    ],
};
