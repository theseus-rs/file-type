use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62522500: FileFormat = FileFormat {
    id: 62_522_500,
    source_type: SourceType::Wikidata,
    name: "SPARQL query file format",
    extensions: &["rq"],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
