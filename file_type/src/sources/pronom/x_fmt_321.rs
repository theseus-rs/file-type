use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_321: FileFormat = FileFormat {
    id: 483,
    puid: "x-fmt/321",
    name: "Framework Database",
    extensions: &["fw", "fw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
