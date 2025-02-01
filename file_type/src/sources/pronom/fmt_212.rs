use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_212: FileFormat = FileFormat {
    id: 938,
    puid: "fmt/212",
    name: "Information or Setup File",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
