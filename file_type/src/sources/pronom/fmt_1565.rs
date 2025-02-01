use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1565: FileFormat = FileFormat {
    id: 2_390,
    puid: "fmt/1565",
    name: "reStructuredText",
    extensions: &["rst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
