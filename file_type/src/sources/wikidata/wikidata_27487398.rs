use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487398: FileFormat = FileFormat {
    id: 27_487_398,
    puid: "wikidata/27487398",
    name: "Shapefile geocoding index, ODB format",
    extensions: &["mxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
