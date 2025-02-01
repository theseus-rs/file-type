use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7178768: FileFormat = FileFormat {
    id: 7_178_768,
    puid: "wikidata/7178768",
    name: "Petri Net Markup Language",
    extensions: &["pnml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
