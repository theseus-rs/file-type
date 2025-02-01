use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_18609762: FileFormat = FileFormat {
    id: 18_609_762,
    puid: "wikidata/18609762",
    name: "SPARQL Query Results JSON Format",
    extensions: &["srj"],
    media_types: &["application/sparql-results+json"],
    internal_signatures: &[],
    related_formats: &[],
};
