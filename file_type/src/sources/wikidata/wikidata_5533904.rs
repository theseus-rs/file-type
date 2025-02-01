use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5533904: FileFormat = FileFormat {
    id: 5_533_904,
    puid: "wikidata/5533904",
    name: "GeoJSON",
    extensions: &["geojson"],
    media_types: &["application/geo+json"],
    internal_signatures: &[],
    related_formats: &[],
};
