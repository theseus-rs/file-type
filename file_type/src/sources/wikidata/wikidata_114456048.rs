use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114456048: FileFormat = FileFormat {
    id: 114_456_048,
    source_type: SourceType::Wikidata,
    name: "Apache Avro Protocol Data",
    extensions: &["avpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
