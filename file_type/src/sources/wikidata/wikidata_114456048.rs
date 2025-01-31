use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114456048: FileFormat = FileFormat {
    id: 114_456_048,
    puid: "wikidata/114456048",
    name: "Apache Avro Protocol Data",
    extensions: &["avpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
