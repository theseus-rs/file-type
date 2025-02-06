use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29465360: FileFormat = FileFormat {
    id: 29_465_360,
    source_type: SourceType::Wikidata,
    name: "VPM",
    extensions: &["vpm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
