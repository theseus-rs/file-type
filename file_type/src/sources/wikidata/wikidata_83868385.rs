use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83868385: FileFormat = FileFormat {
    id: 83_868_385,
    puid: "wikidata/83868385",
    name: "SOSI, version 4.5",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[],
    related_formats: &[],
};
