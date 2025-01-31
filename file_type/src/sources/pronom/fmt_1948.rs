use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1948: FileFormat = FileFormat {
    id: 2_810,
    puid: "fmt/1948",
    name: "Common Data Format dotCDF",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_812,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
