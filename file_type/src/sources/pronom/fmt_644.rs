use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_644: FileFormat = FileFormat {
    id: 1_443,
    puid: "fmt/644",
    name: "Nullsoft Scriptable Install System",
    extensions: &["nsi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
