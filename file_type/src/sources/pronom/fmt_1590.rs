use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1590: FileFormat = FileFormat {
    id: 2_416,
    puid: "fmt/1590",
    name: "Visual Basic Binary Form File",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
