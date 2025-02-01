use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_446: FileFormat = FileFormat {
    id: 861,
    puid: "x-fmt/446",
    name: "Revit External Group",
    extensions: &["rvg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 767,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
