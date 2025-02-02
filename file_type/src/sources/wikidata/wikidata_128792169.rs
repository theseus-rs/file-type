use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128792169: FileFormat = FileFormat {
    id: 128_792_169,
    source_type: SourceType::Wikidata,
    name: "Cypher query format",
    extensions: &["cyp", "cypher"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
