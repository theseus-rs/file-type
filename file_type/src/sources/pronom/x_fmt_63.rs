use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_63: FileFormat = FileFormat {
    id: 104,
    puid: "x-fmt/63",
    name: "AutoLISP File",
    extensions: &["lsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
