use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_114: FileFormat = FileFormat {
    id: 727,
    puid: "fmt/114",
    name: "Windows Bitmap",
    extensions: &["ddb", "bmp"],
    media_types: &["image/bmp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(6),
                    Token::Literal(&[0x01]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x08])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 728,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
