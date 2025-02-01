use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692911: FileFormat = FileFormat {
    id: 125_692_911,
    puid: "wikidata/125692911",
    name: "StarImpress 4.0/5.0",
    extensions: &["sdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
