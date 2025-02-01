use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_245: FileFormat = FileFormat {
    id: 977,
    puid: "fmt/245",
    name: "Structured Data Exchange Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
