use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83868357: FileFormat = FileFormat {
    id: 83_868_357,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 4",
    extensions: &["sos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
