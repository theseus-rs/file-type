use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1340077: FileFormat = FileFormat {
    id: 1_340_077,
    puid: "wikidata/1340077",
    name: "Encoded Archival Description",
    extensions: &["sgm", "sgml"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
