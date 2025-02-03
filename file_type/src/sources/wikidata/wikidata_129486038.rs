use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129486038: FileFormat = FileFormat {
    id: 129_486_038,
    source_type: SourceType::Wikidata,
    name: "DOT language file format",
    extensions: &["dot"],
    media_types: &["text/vnd.graphviz", "text/x-graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
