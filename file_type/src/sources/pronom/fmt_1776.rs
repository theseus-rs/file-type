use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1776: FileFormat = FileFormat {
    id: 2_626,
    puid: "fmt/1776",
    name: "Extensible Markup Language",
    extensions: &["xml"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C]),
                    Token::Any(&[
                        &[Token::Literal(&[0x20])],
                        &[Token::Literal(&[0x09])],
                        &[Token::Literal(&[0x0A])],
                        &[Token::Literal(&[0x0D])],
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x3D]),
                    Token::WildcardCountRange(0, 1),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x31, 0x2E, 0x31]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
        RelatedFormat {
            id: 931,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
