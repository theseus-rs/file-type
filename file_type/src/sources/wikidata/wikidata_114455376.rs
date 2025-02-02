use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114455376: FileFormat = FileFormat {
    id: 114_455_376,
    source_type: SourceType::Wikidata,
    name: "Apache Avro Schema file format",
    extensions: &["avsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
