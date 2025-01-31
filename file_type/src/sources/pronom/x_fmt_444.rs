use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_444: FileFormat = FileFormat {
    id: 858,
    puid: "x-fmt/444",
    name: "Revit Family Template",
    extensions: &["rft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 767,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
