use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2185: FileFormat = FileFormat {
    id: 2_185,
    source_type: SourceType::Pronom,
    name: "GeoJSON",
    extensions: &["geojson"],
    media_types: &["application/geo+json"],
    signatures: &[],
    related_formats: &[],
};
