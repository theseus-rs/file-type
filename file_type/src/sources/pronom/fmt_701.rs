use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_701: FileFormat = FileFormat {
    id: 1_500,
    puid: "fmt/701",
    name: "Processing Development Environment",
    extensions: &["pde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
