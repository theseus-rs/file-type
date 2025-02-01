use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_751: FileFormat = FileFormat {
    id: 1_550,
    puid: "fmt/751",
    name: "AppleWorks Database",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x06]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                    Token::WildcardCount(270),
                    Token::Literal(&[0x03]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_545,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
