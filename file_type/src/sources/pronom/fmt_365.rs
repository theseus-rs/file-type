use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_365: FileFormat = FileFormat {
    id: 1_112,
    puid: "fmt/365",
    name: "National Imagery Transmission Format",
    extensions: &["ntf"],
    media_types: &["application/vnd.nitf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4E, 0x49, 0x54, 0x46, 0x30, 0x32, 0x2E, 0x30, 0x30]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x31], &[0x39]),
                    Token::Literal(&[0x20, 0x20, 0x20, 0x20]),
                    Token::WildcardCount(10),
                    Token::Range(&[0x30], &[0x33]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x32]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x5A]),
                    Token::WildcardCount(3),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::WildcardCount(80),
                    Token::Any(&[
                        &[Token::Literal(&[0x43])],
                        &[Token::Literal(&[0x52])],
                        &[Token::Literal(&[0x53])],
                        &[Token::Literal(&[0x54])],
                        &[Token::Literal(&[0x55])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_113,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_111,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
