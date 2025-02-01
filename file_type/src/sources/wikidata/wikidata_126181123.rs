use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126181123: FileFormat = FileFormat {
    id: 126_181_123,
    puid: "wikidata/126181123",
    name: "Graphisoft Archicad Project 6-9",
    extensions: &["pla", "pln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
