use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857382: FileFormat = FileFormat {
    id: 105_857_382,
    source_type: SourceType::Wikidata,
    name: "Node-RED flow",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
