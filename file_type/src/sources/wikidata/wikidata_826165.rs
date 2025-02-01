use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_826165: FileFormat = FileFormat {
    id: 826_165,
    puid: "wikidata/826165",
    name: "Web Ontology Language",
    extensions: &["owl"],
    media_types: &["application/owl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
