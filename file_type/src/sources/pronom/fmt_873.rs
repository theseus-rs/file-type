use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_873: FileFormat = FileFormat {
    id: 1_677,
    puid: "fmt/873",
    name: "Notation3",
    extensions: &["n3"],
    media_types: &["text/n3"],
    internal_signatures: &[],
    related_formats: &[],
};
