use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1007: FileFormat = FileFormat {
    id: 1_812,
    puid: "fmt/1007",
    name: "Digital Speech Standard",
    extensions: &["dss"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x64, 0x73, 0x73])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_813,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
