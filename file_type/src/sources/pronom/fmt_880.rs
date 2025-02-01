use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_880: FileFormat = FileFormat {
    id: 1_684,
    puid: "fmt/880",
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
