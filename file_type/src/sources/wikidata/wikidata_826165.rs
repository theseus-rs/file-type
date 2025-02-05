use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_826165: FileFormat = FileFormat {
    id: 826_165,
    source_type: SourceType::Wikidata,
    name: "Web Ontology Language",
    extensions: &["owl"],
    media_types: &["application/owl+xml"],
    signatures: &[],
    related_formats: &[],
};
