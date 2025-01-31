use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_402: FileFormat = FileFormat {
    id: 756,
    puid: "x-fmt/402",
    name: "StarOffice Draw",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
