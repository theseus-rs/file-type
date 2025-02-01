use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_194: FileFormat = FileFormat {
    id: 267,
    puid: "x-fmt/194",
    name: "IRIS Graphics",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
