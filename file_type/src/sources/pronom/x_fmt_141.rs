use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_141: FileFormat = FileFormat {
    id: 202,
    puid: "x-fmt/141",
    name: "Calendar Creator Plus Data File",
    extensions: &["cce"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0xCC]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x29]),
                    Token::WildcardCount(43),
                    Token::Literal(&[0x65, 0x00, 0x7F, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_113,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
