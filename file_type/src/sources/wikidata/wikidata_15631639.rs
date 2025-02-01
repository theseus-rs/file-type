use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15631639: FileFormat = FileFormat {
    id: 15_631_639,
    puid: "wikidata/15631639",
    name: "qgs",
    extensions: &["qgs"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
