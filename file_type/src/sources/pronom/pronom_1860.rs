use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1860: FileFormat = FileFormat {
    id: 1_860,
    source_type: SourceType::Pronom,
    name: "M2TS",
    extensions: &["mts", "m2ts"],
    media_types: &[],
    signatures: &[Signature {
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
