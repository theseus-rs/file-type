use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114455550: FileFormat = FileFormat {
    id: 114_455_550,
    source_type: SourceType::Wikidata,
    name: "Apache Avro IDL Data",
    extensions: &["avdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
