use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1055: FileFormat = FileFormat {
    id: 1_860,
    puid: "fmt/1055",
    name: "M2TS",
    extensions: &["mts", "m2ts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(191),
                    Token::Literal(&[0x47]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
