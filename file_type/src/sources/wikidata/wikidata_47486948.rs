use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47486948: FileFormat = FileFormat {
    id: 47_486_948,
    puid: "wikidata/47486948",
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
