use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206210: FileFormat = FileFormat {
    id: 28_206_210,
    puid: "wikidata/28206210",
    name: "Graph Saurus SR7/SR8/SRS",
    extensions: &["sr7", "sr8", "srs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
