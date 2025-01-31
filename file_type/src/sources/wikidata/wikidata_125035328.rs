use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125035328: FileFormat = FileFormat {
    id: 125_035_328,
    puid: "wikidata/125035328",
    name: "TinkerPlots document",
    extensions: &["tp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
