use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857382: FileFormat = FileFormat {
    id: 105_857_382,
    puid: "wikidata/105857382",
    name: "Node-RED flow",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
