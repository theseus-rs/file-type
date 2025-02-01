use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29465360: FileFormat = FileFormat {
    id: 29_465_360,
    puid: "wikidata/29465360",
    name: "VPM",
    extensions: &["vpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
