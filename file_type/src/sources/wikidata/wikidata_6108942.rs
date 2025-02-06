use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6108942: FileFormat = FileFormat {
    id: 6_108_942,
    source_type: SourceType::Wikidata,
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &["application/ld+json"],
    signatures: &[],
    related_formats: &[],
};
