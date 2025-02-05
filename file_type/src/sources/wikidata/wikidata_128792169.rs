use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128792169: FileFormat = FileFormat {
    id: 128_792_169,
    source_type: SourceType::Wikidata,
    name: "Cypher query format",
    extensions: &["cyp", "cypher"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
