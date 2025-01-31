use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_658: FileFormat = FileFormat {
    id: 1_457,
    puid: "fmt/658",
    name: "Cypher Query Language",
    extensions: &["cql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
