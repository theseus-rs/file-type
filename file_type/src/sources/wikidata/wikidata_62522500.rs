use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62522500: FileFormat = FileFormat {
    id: 62_522_500,
    puid: "wikidata/62522500",
    name: "SPARQL query file format",
    extensions: &["rq"],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
