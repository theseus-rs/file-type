use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_900927: FileFormat = FileFormat {
    id: 900_927,
    source_type: SourceType::Wikidata,
    name: "DOT language",
    extensions: &["dot", "gv"],
    media_types: &["text/vnd.graphviz"],
    signatures: &[],
    related_formats: &[],
};
