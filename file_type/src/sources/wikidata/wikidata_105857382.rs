use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857382: FileFormat = FileFormat {
    id: 105_857_382,
    source_type: SourceType::Wikidata,
    name: "Node-RED flow",
    extensions: &["json"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
