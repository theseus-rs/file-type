use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123349564: FileFormat = FileFormat {
    id: 123_349_564,
    puid: "wikidata/123349564",
    name: "Clooz database file",
    extensions: &["clz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
