use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_391: FileFormat = FileFormat {
    id: 1_139,
    puid: "fmt/391",
    name: "Log ASCII Standard Format",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x7E, 0x56]),
                    Token::Any(&[
                        &[Token::Literal(&[0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E])],
                        &[Token::Literal(&[0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E])],
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x2E]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x33, 0x2E, 0x30]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x3A]),
                    Token::WildcardCountRange(0, 3),
                    Token::Literal(&[0x43, 0x57, 0x4C, 0x53, 0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x4C, 0x4F, 0x47, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20, 0x53, 0x54,
                            0x41, 0x4E, 0x44, 0x41, 0x52, 0x44,
                        ])],
                        &[Token::Literal(&[0x4C, 0x41, 0x53])],
                    ]),
                    Token::WildcardCountRange(1, 3),
                    Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x33, 0x2E, 0x30,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x7E, 0x57]),
                    Token::Any(&[
                        &[Token::Literal(&[0x45, 0x4C, 0x4C])],
                        &[Token::Literal(&[0x65, 0x6C, 0x6C])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_138,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
