use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_874: FileFormat = FileFormat {
    id: 1_678,
    puid: "fmt/874",
    name: "Turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    internal_signatures: &[],
    related_formats: &[],
};
