use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130712861: FileFormat = FileFormat {
    id: 130_712_861,
    puid: "wikidata/130712861",
    name: "Relation Query Language file format",
    extensions: &["rql"],
    media_types: &["text/x-rql"],
    internal_signatures: &[],
    related_formats: &[],
};
