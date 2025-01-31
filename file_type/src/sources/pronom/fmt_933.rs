use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_933: FileFormat = FileFormat {
    id: 1_738,
    puid: "fmt/933",
    name: "Simple Vector Format",
    extensions: &["svf"],
    media_types: &["image/vnd-svf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x56, 0x46, 0x20, 0x76, 0x31])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_739,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
