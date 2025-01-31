use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128792169: FileFormat = FileFormat {
    id: 128_792_169,
    puid: "wikidata/128792169",
    name: "Cypher query format",
    extensions: &["cyp", "cypher"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
