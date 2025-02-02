use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29465360: FileFormat = FileFormat {
    id: 29_465_360,
    source_type: SourceType::Wikidata,
    name: "VPM",
    extensions: &["vpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
