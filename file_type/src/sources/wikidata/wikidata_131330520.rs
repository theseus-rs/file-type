use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131330520: FileFormat = FileFormat {
    id: 131_330_520,
    puid: "wikidata/131330520",
    name: "Typographic Number Theory file format",
    extensions: &["tnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
