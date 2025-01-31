use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62446408: FileFormat = FileFormat {
    id: 62_446_408,
    puid: "wikidata/62446408",
    name: "OWL Manchester Syntax",
    extensions: &["omn"],
    media_types: &["text/owl-manchester"],
    internal_signatures: &[],
    related_formats: &[],
};
