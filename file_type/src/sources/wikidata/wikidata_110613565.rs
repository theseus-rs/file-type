use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110613565: FileFormat = FileFormat {
    id: 110_613_565,
    source_type: SourceType::Wikidata,
    name: "Mapbox Vector Tiles",
    extensions: &["mvt"],
    media_types: &["application/vnd.mapbox-vector-tile"],
    signatures: &[],
    related_formats: &[],
};
