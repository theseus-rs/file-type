use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1543: FileFormat = FileFormat {
    id: 2_368,
    puid: "fmt/1543",
    name: "ELAN Annotation File",
    extensions: &["eaf"],
    media_types: &["application/eaf+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(27),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x4E, 0x4E, 0x4F, 0x54, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x5F, 0x44,
                            0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E, 0x54, 0x20,
                        ]),
                        Token::WildcardCountRange(0, 256),
                        Token::Literal(&[0x41, 0x55, 0x54, 0x48, 0x4F, 0x52, 0x3D]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x2F, 0x41, 0x4E, 0x4E, 0x4F, 0x54, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                            0x5F, 0x44, 0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E, 0x54, 0x3E,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0D, 0x0A])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
