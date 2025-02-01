use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130368377: FileFormat = FileFormat {
    id: 130_368_377,
    puid: "wikidata/130368377",
    name: "nesC source code file",
    extensions: &["nc"],
    media_types: &["text/x-nescsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
