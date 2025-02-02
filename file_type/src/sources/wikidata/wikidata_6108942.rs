use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6108942: FileFormat = FileFormat {
    id: 6_108_942,
    source_type: SourceType::Wikidata,
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &["application/ld+json"],
    internal_signatures: &[],
    related_formats: &[],
};
