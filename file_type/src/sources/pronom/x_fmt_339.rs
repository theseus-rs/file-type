use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_339: FileFormat = FileFormat {
    id: 503,
    puid: "x-fmt/339",
    name: "Lotus Notes File",
    extensions: &["box"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
