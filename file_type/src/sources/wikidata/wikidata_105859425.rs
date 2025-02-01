use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859425: FileFormat = FileFormat {
    id: 105_859_425,
    puid: "wikidata/105859425",
    name: "QuakeML seismological data",
    extensions: &["quakeml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
