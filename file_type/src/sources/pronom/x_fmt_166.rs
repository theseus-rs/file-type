use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_166: FileFormat = FileFormat {
    id: 238,
    puid: "x-fmt/166",
    name: "PICS Animation",
    extensions: &["pcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
