use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857388: FileFormat = FileFormat {
    id: 105_857_388,
    source_type: SourceType::Wikidata,
    name: "JSON Playlist File",
    extensions: &["jspf"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
