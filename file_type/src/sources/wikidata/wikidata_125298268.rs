use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125298268: FileFormat = FileFormat {
    id: 125_298_268,
    puid: "wikidata/125298268",
    name: "Scribble",
    extensions: &["scrbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
