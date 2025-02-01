use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_208: FileFormat = FileFormat {
    id: 934,
    puid: "fmt/208",
    name: "Binary File",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_439,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
