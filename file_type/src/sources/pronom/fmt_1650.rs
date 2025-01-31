use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1650: FileFormat = FileFormat {
    id: 2_477,
    puid: "fmt/1650",
    name: "Bayesian Interchange Format File",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
