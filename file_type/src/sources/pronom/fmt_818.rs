use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_818: FileFormat = FileFormat {
    id: 1_618,
    puid: "fmt/818",
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
