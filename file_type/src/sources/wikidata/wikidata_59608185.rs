use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59608185: FileFormat = FileFormat {
    id: 59_608_185,
    puid: "wikidata/59608185",
    name: "Media View Pro",
    extensions: &["mpcatalog"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
