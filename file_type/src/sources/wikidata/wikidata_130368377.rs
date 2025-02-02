use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130368377: FileFormat = FileFormat {
    id: 130_368_377,
    source_type: SourceType::Wikidata,
    name: "nesC source code file",
    extensions: &["nc"],
    media_types: &["text/x-nescsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
