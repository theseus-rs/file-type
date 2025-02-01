use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857388: FileFormat = FileFormat {
    id: 105_857_388,
    puid: "wikidata/105857388",
    name: "JSON Playlist File",
    extensions: &["jspf"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
