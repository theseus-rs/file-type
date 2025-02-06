use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114455550: FileFormat = FileFormat {
    id: 114_455_550,
    source_type: SourceType::Wikidata,
    name: "Apache Avro IDL Data",
    extensions: &["avdl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
