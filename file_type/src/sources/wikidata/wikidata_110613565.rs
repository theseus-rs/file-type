use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110613565: FileFormat = FileFormat {
    id: 110_613_565,
    puid: "wikidata/110613565",
    name: "Mapbox Vector Tiles",
    extensions: &["mvt"],
    media_types: &["application/vnd.mapbox-vector-tile"],
    internal_signatures: &[],
    related_formats: &[],
};
