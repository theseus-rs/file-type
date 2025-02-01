use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1315: FileFormat = FileFormat {
    id: 2_133,
    puid: "fmt/1315",
    name: "GL Transmission Format (Text)",
    extensions: &["gltf"],
    media_types: &["application/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x7B]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x22, 0x61, 0x73, 0x73, 0x65, 0x74, 0x22]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x7B]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22]),
                        Token::WildcardCountRange(0, 20),
                        Token::Literal(&[0x3A]),
                        Token::WildcardCountRange(0, 20),
                        Token::Literal(&[0x22, 0x32, 0x2E]),
                        Token::WildcardCountRange(1, 10),
                        Token::Literal(&[0x22]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7D])],
                },
            },
        ],
    }],
    related_formats: &[],
};
