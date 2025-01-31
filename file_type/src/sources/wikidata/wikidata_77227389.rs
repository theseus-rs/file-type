use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77227389: FileFormat = FileFormat {
    id: 77_227_389,
    puid: "wikidata/77227389",
    name: "Bayesian Networks Interchange Format",
    extensions: &["bifxml", "xmlbif"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
