use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_18609754: FileFormat = FileFormat {
    id: 18_609_754,
    source_type: SourceType::Wikidata,
    name: "SPARQL Query Results XML Format",
    extensions: &["srx"],
    media_types: &["application/sparql-results+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
