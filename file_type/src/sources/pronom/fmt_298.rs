use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_298: FileFormat = FileFormat {
    id: 1_042,
    puid: "fmt/298",
    name: "Autodesk Animator Pro FLIC",
    extensions: &["flc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x12, 0xAF]),
                    Token::WildcardCount(6),
                    Token::Literal(&[0x08]),
                    Token::WildcardCount(120),
                    Token::Literal(&[0xF1]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
