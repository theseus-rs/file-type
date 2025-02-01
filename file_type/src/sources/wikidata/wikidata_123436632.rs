use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123436632: FileFormat = FileFormat {
    id: 123_436_632,
    puid: "wikidata/123436632",
    name: "Construction File",
    extensions: &["cf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
