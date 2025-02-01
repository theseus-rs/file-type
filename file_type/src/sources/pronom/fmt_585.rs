use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_585: FileFormat = FileFormat {
    id: 1_373,
    puid: "fmt/585",
    name: "MPEG-2 Transport Stream",
    extensions: &["m2t", "ts", "m2ts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
