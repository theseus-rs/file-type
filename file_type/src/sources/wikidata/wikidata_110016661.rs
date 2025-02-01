use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110016661: FileFormat = FileFormat {
    id: 110_016_661,
    puid: "wikidata/110016661",
    name: "TEI P4 XML - Corpus File",
    extensions: &["odd", "tei", "xml"],
    media_types: &[
        "application/tei+xml",
        "application/tei+xml",
        "application/tei+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
