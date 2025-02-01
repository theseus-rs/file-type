use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110016211: FileFormat = FileFormat {
    id: 110_016_211,
    puid: "wikidata/110016211",
    name: "TEI P4 XML - Single Text File",
    extensions: &["odd", "tei", "xml"],
    media_types: &[
        "application/tei+xml",
        "application/tei+xml",
        "application/tei+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
