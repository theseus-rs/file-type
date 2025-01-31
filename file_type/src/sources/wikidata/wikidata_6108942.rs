use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6108942: FileFormat = FileFormat {
    id: 6_108_942,
    puid: "wikidata/6108942",
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &["application/ld+json"],
    internal_signatures: &[],
    related_formats: &[],
};
