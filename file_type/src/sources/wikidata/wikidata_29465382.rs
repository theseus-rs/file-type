use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29465382: FileFormat = FileFormat {
    id: 29_465_382,
    puid: "wikidata/29465382",
    name: "UltraEdit Project User Interface",
    extensions: &["pui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
