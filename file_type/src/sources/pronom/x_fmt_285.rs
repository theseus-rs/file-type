use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_285: FileFormat = FileFormat {
    id: 436,
    puid: "x-fmt/285",
    name: "IBM DisplayWrite Revisable Form Text File",
    extensions: &["rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
