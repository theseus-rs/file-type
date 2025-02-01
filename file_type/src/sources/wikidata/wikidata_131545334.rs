use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131545334: FileFormat = FileFormat {
    id: 131_545_334,
    puid: "wikidata/131545334",
    name: "GeoArrow file format",
    extensions: &["arrow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
