use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487388: FileFormat = FileFormat {
    id: 27_487_388,
    puid: "wikidata/27487388",
    name: "Shapefile geocoding index",
    extensions: &["ixs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
