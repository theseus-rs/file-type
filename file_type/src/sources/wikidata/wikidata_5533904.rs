use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5533904: FileFormat = FileFormat {
    id: 5_533_904,
    source_type: SourceType::Wikidata,
    name: "GeoJSON",
    extensions: &["geojson"],
    media_types: &["application/geo+json"],
    signatures: &[],
    related_formats: &[],
};
