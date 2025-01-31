use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59608283: FileFormat = FileFormat {
    id: 59_608_283,
    puid: "wikidata/59608283",
    name: "KryoFlux 2 format",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
