use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1763: FileFormat = FileFormat {
    id: 2_613,
    puid: "fmt/1763",
    name: "MacBinary",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(73),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(7),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(19),
                    Token::Literal(&[0x6D, 0x42, 0x49, 0x4E]),
                    Token::WildcardCount(16),
                    Token::Literal(&[0x82]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_612,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
