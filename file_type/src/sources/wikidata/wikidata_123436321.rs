use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123436321: FileFormat = FileFormat {
    id: 123_436_321,
    puid: "wikidata/123436321",
    name: "DARC-F1 Query file",
    extensions: &["f1q"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
