use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_442: FileFormat = FileFormat {
    id: 856,
    puid: "x-fmt/442",
    name: "form*Z Project File",
    extensions: &["fmz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
