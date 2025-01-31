use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_22: FileFormat = FileFormat {
    id: 51,
    puid: "x-fmt/22",
    name: "7-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
