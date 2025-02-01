use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_118: FileFormat = FileFormat {
    id: 731,
    puid: "fmt/118",
    name: "Windows Bitmap",
    extensions: &["bmp", "dib"],
    media_types: &["image/bmp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x6C, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(8),
                    Token::Literal(&[0x01, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x08])],
                        &[Token::Literal(&[0x10])],
                        &[Token::Literal(&[0x18])],
                        &[Token::Literal(&[0x20])],
                    ]),
                    Token::Literal(&[0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                    ]),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 732,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 729,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 730,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
