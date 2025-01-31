use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48940: FileFormat = FileFormat {
    id: 48_940,
    puid: "wikidata/48940",
    name: "RDF/XML",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
