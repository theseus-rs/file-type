use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48940: FileFormat = FileFormat {
    id: 48_940,
    source_type: SourceType::Wikidata,
    name: "RDF/XML",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
