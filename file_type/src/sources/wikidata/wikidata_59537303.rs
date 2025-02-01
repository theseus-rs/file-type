use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59537303: FileFormat = FileFormat {
    id: 59_537_303,
    puid: "wikidata/59537303",
    name: "Nullsoft Scriptable Install System",
    extensions: &["nsi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
