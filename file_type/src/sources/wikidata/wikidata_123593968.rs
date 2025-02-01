use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123593968: FileFormat = FileFormat {
    id: 123_593_968,
    puid: "wikidata/123593968",
    name: "SGI Movie File",
    extensions: &["movie", "mv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
