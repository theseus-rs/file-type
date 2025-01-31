use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125077026: FileFormat = FileFormat {
    id: 125_077_026,
    puid: "wikidata/125077026",
    name: "Gregorian chant score file",
    extensions: &["gabc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
