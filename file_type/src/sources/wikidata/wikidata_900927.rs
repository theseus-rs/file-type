use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_900927: FileFormat = FileFormat {
    id: 900_927,
    puid: "wikidata/900927",
    name: "DOT language",
    extensions: &["dot", "gv"],
    media_types: &["text/vnd.graphviz", "text/vnd.graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
