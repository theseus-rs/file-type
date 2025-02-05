use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206210: FileFormat = FileFormat {
    id: 28_206_210,
    source_type: SourceType::Wikidata,
    name: "Graph Saurus SR7/SR8/SRS",
    extensions: &["sr7", "sr8", "srs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
