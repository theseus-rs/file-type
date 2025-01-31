use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1761: FileFormat = FileFormat {
    id: 2_611,
    puid: "fmt/1761",
    name: "MacBinary",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_612,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
