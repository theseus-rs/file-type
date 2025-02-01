use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47486941: FileFormat = FileFormat {
    id: 47_486_941,
    puid: "wikidata/47486941",
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
