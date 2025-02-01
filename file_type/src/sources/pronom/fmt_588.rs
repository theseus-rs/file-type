use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_588: FileFormat = FileFormat {
    id: 1_380,
    puid: "fmt/588",
    name: "Redcode RAW (R3D) Media File",
    extensions: &["r3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x45, 0x44, 0x31]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x52, 0x31]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_843,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
