use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1625: FileFormat = FileFormat {
    id: 2_452,
    puid: "fmt/1625",
    name: "ESRI Colour File Format",
    extensions: &["clr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
