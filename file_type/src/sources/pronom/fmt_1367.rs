use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1367: FileFormat = FileFormat {
    id: 2_185,
    puid: "fmt/1367",
    name: "GeoJSON",
    extensions: &["geojson"],
    media_types: &["application/geo+json"],
    internal_signatures: &[],
    related_formats: &[],
};
