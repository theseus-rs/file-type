use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1451: FileFormat = FileFormat {
    id: 2_272,
    puid: "fmt/1451",
    name: "PDF Portfolio",
    extensions: &["pdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x37]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x2F, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x25, 0x45, 0x4F]),
                        Token::Any(&[
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x46, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D])],
                            &[Token::Literal(&[0x46, 0x0D, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D, 0x00])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 1_016,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
