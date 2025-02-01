use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_752: FileFormat = FileFormat {
    id: 1_551,
    puid: "fmt/752",
    name: "AppleWorks Painting",
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
                    Token::Literal(&[0x04]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_546,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
