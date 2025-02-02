use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_826165: FileFormat = FileFormat {
    id: 826_165,
    source_type: SourceType::Wikidata,
    name: "Web Ontology Language",
    extensions: &["owl"],
    media_types: &["application/owl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
