use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83868357: FileFormat = FileFormat {
    id: 83_868_357,
    puid: "wikidata/83868357",
    name: "SOSI, version 4",
    extensions: &["sos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
