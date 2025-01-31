use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113472408: FileFormat = FileFormat {
    id: 113_472_408,
    puid: "wikidata/113472408",
    name: "Glyphs Character Data",
    extensions: &["glyphs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
