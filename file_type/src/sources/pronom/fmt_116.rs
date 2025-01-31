use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_116: FileFormat = FileFormat {
    id: 729,
    puid: "fmt/116",
    name: "Windows Bitmap",
    extensions: &["dib", "bmp"],
    media_types: &["image/bmp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x4D]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x28, 0x00, 0x00, 0x00]),
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
                    ]),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 731,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 728,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
