use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1856: FileFormat = FileFormat {
    id: 2_710,
    puid: "fmt/1856",
    name: "Enhanced Image Package",
    extensions: &["eip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
