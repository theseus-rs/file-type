use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_184: FileFormat = FileFormat {
    id: 905,
    puid: "fmt/184",
    name: "PrimeOCR",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x33, 0x38, 0x30, 0x2C]),
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
