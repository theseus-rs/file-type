use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129486038: FileFormat = FileFormat {
    id: 129_486_038,
    puid: "wikidata/129486038",
    name: "DOT language file format",
    extensions: &["dot", "dot"],
    media_types: &["text/vnd.graphviz", "text/x-graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
