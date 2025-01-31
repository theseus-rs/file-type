use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114455376: FileFormat = FileFormat {
    id: 114_455_376,
    puid: "wikidata/114455376",
    name: "Apache Avro Schema file format",
    extensions: &["avsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
