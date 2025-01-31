use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10846539: FileFormat = FileFormat {
    id: 10_846_539,
    puid: "wikidata/10846539",
    name: "BNA",
    extensions: &["bna"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
