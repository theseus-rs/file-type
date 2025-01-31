use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_365: FileFormat = FileFormat {
    id: 531,
    puid: "x-fmt/365",
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
