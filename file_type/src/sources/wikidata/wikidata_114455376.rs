use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114455376: FileFormat = FileFormat {
    id: 114_455_376,
    source_type: SourceType::Wikidata,
    name: "Apache Avro Schema file format",
    extensions: &["avsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
