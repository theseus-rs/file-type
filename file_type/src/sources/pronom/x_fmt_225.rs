use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_225: FileFormat = FileFormat {
    id: 317,
    puid: "x-fmt/225",
    name: "ESRI MapInfo Data File",
    extensions: &["mid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
