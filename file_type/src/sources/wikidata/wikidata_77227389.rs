use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_77227389: FileFormat = FileFormat {
    id: 77_227_389,
    source_type: SourceType::Wikidata,
    name: "Bayesian Networks Interchange Format",
    extensions: &["bifxml", "xmlbif"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
