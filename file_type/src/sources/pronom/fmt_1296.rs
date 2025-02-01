use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1296: FileFormat = FileFormat {
    id: 2_114,
    puid: "fmt/1296",
    name: "Calendar Creator File",
    extensions: &["cc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_115,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
