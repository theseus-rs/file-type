use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000657: FileFormat = FileFormat {
    id: 29_000_657,
    puid: "wikidata/29000657",
    name: "Polygon data file",
    extensions: &["poly"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
