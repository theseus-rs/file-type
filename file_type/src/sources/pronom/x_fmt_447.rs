use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_447: FileFormat = FileFormat {
    id: 862,
    puid: "x-fmt/447",
    name: "Revit Project",
    extensions: &["rvt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 767,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
