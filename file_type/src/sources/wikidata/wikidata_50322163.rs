use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50322163: FileFormat = FileFormat {
    id: 50_322_163,
    puid: "wikidata/50322163",
    name: "RDF/JSON",
    extensions: &["rj"],
    media_types: &["application/rdf+json"],
    internal_signatures: &[],
    related_formats: &[],
};
