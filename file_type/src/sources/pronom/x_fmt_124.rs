use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_124: FileFormat = FileFormat {
    id: 179,
    puid: "x-fmt/124",
    name: "Microsoft Excel Add-In",
    extensions: &["xla", "xll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
