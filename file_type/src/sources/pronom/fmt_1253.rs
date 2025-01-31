use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1253: FileFormat = FileFormat {
    id: 2_071,
    puid: "fmt/1253",
    name: "ESRI Code Page File",
    extensions: &["cpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
