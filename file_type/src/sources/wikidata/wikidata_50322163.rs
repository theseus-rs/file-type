use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50322163: FileFormat = FileFormat {
    id: 50_322_163,
    source_type: SourceType::Wikidata,
    name: "RDF/JSON",
    extensions: &["rj"],
    media_types: &["application/rdf+json"],
    signatures: &[],
    related_formats: &[],
};
