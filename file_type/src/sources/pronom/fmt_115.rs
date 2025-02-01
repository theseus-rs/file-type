use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_115: FileFormat = FileFormat {
    id: 728,
    puid: "fmt/115",
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
                    Token::Literal(&[0x0C, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x01, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x08])],
                        &[Token::Literal(&[0x18])],
                    ]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 55,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 729,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 730,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 727,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
