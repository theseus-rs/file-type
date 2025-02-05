use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1543319: FileFormat = FileFormat {
    id: 1_543_319,
    source_type: SourceType::Wikidata,
    name: "Graph Modelling Language",
    extensions: &["gml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
