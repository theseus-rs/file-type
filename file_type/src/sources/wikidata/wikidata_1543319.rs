use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1543319: FileFormat = FileFormat {
    id: 1_543_319,
    puid: "wikidata/1543319",
    name: "Graph Modelling Language",
    extensions: &["gml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
