use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_138: FileFormat = FileFormat {
    id: 198,
    puid: "x-fmt/138",
    name: "Active Server Page",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
