use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50322163: FileFormat = FileFormat {
    id: 50_322_163,
    source_type: SourceType::Wikidata,
    name: "RDF/JSON",
    extensions: &["rj"],
    media_types: &["application/rdf+json"],
    internal_signatures: &[],
    related_formats: &[],
};
