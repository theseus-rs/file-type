use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_884: FileFormat = FileFormat {
    id: 1_688,
    puid: "fmt/884",
    name: "AXD HTTP Handler File",
    extensions: &["axd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
