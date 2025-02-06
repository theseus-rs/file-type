use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129486038: FileFormat = FileFormat {
    id: 129_486_038,
    source_type: SourceType::Wikidata,
    name: "DOT language file format",
    extensions: &["dot"],
    media_types: &["text/vnd.graphviz", "text/x-graphviz"],
    signatures: &[],
    related_formats: &[],
};
