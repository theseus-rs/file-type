use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_822: FileFormat = FileFormat {
    id: 822,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Email Message",
    extensions: &["msg", "oft"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0xFE, 0xFF]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x50, 0x4D, 0x2E, 0x4E, 0x6F, 0x74, 0x65,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x00, 0x65, 0x00, 0x63, 0x00, 0x69, 0x00, 0x70, 0x00, 0x5F, 0x00,
                        0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00, 0x6F, 0x00,
                        0x6E, 0x00,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 767,
    }],
};
