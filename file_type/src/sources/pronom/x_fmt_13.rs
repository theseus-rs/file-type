use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_13: FileFormat = FileFormat {
    id: 40,
    puid: "x-fmt/13",
    name: "Tab-separated Values",
    extensions: &["tsv", "tab"],
    media_types: &["text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
