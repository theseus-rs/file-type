use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_283: FileFormat = FileFormat {
    id: 434,
    puid: "x-fmt/283",
    name: "8-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
