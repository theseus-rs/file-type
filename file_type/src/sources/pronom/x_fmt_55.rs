use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_55: FileFormat = FileFormat {
    id: 89,
    puid: "x-fmt/55",
    name: "Frame Vector Metafile",
    extensions: &["fmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
