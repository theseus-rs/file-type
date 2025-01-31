use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857202: FileFormat = FileFormat {
    id: 105_857_202,
    puid: "wikidata/105857202",
    name: "Hydrogen song",
    extensions: &["h2song"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
