use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_443: FileFormat = FileFormat {
    id: 857,
    puid: "x-fmt/443",
    name: "Revit Family File",
    extensions: &["rfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 767,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
