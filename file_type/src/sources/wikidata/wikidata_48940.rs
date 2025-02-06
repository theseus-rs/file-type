use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48940: FileFormat = FileFormat {
    id: 48_940,
    source_type: SourceType::Wikidata,
    name: "RDF/XML",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    signatures: &[],
    related_formats: &[],
};
