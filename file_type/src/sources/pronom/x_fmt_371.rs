use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_371: FileFormat = FileFormat {
    id: 547,
    puid: "x-fmt/371",
    name: "XYWrite for Windows Document",
    extensions: &["xyw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
