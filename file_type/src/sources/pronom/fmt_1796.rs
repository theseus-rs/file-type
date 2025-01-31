use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1796: FileFormat = FileFormat {
    id: 2_647,
    puid: "fmt/1796",
    name: "Wireless Markup Language (WML) Document",
    extensions: &["wml"],
    media_types: &["text/vnd.wap.wml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22,
                        ]),
                        Token::WildcardCountRange(0, 75),
                        Token::Literal(&[
                            0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x77, 0x6D,
                            0x6C, 0x20, 0x50, 0x55, 0x42, 0x4C, 0x49, 0x43, 0x20, 0x22, 0x2D, 0x2F,
                            0x2F, 0x57, 0x41, 0x50, 0x46, 0x4F, 0x52, 0x55, 0x4D, 0x2F, 0x2F, 0x44,
                            0x54, 0x44, 0x20, 0x57, 0x4D, 0x4C,
                        ]),
                        Token::WildcardCountRange(20, 200),
                        Token::Literal(&[0x3C, 0x77, 0x6D, 0x6C, 0x3E]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x2F, 0x63, 0x61, 0x72, 0x64, 0x3E]),
                        Token::WildcardCountRange(0, 6),
                        Token::Literal(&[0x3C, 0x2F, 0x77, 0x6D, 0x6C, 0x3E]),
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
