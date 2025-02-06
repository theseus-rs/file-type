use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_18609762: FileFormat = FileFormat {
    id: 18_609_762,
    source_type: SourceType::Wikidata,
    name: "SPARQL Query Results JSON Format",
    extensions: &["srj"],
    media_types: &["application/sparql-results+json"],
    signatures: &[],
    related_formats: &[],
};
