use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130368377: FileFormat = FileFormat {
    id: 130_368_377,
    source_type: SourceType::Wikidata,
    name: "nesC source code file",
    extensions: &["nc"],
    media_types: &["text/x-nescsrc"],
    signatures: &[],
    related_formats: &[],
};
