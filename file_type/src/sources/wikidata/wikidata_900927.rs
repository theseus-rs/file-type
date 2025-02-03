use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_900927: FileFormat = FileFormat {
    id: 900_927,
    source_type: SourceType::Wikidata,
    name: "DOT language",
    extensions: &["dot", "gv"],
    media_types: &["text/vnd.graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
