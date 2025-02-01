use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_448: FileFormat = FileFormat {
    id: 863,
    puid: "x-fmt/448",
    name: "Revit Workspace",
    extensions: &["rws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
