use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207044: FileFormat = FileFormat {
    id: 28_207_044,
    puid: "wikidata/28207044",
    name: "PM",
    extensions: &["pm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
