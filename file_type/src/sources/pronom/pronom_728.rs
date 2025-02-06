use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_728: FileFormat = FileFormat {
    id: 728,
    source_type: SourceType::Pronom,
    name: "Windows Bitmap",
    extensions: &["bmp", "dib"],
    media_types: &["image/bmp"],
    signatures: &[Signature {
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
            relationship_type: RelationshipType::EquivalentTo,
            id: 55,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 729,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 730,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 727,
        },
    ],
};
