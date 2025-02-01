use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_61: FileFormat = FileFormat {
    id: 102,
    puid: "x-fmt/61",
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
