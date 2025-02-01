use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_445: FileFormat = FileFormat {
    id: 859,
    puid: "x-fmt/445",
    name: "Revit Template",
    extensions: &["rte"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 767,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
