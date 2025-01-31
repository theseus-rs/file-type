use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_185: FileFormat = FileFormat {
    id: 906,
    puid: "fmt/185",
    name: "Prime OCR",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x33, 0x39, 0x30, 0x2C]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x2C]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x2C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
