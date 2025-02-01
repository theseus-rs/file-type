use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125869754: FileFormat = FileFormat {
    id: 125_869_754,
    puid: "wikidata/125869754",
    name: "Pro Tools Session File 5-9",
    extensions: &["ptf", "pts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
