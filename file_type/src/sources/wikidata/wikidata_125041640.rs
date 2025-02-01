use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125041640: FileFormat = FileFormat {
    id: 125_041_640,
    puid: "wikidata/125041640",
    name: "Yoshimi Instrument File",
    extensions: &["xiy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
