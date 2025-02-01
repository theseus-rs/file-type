use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_322: FileFormat = FileFormat {
    id: 485,
    puid: "x-fmt/322",
    name: "Framework Database",
    extensions: &["fw3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
