use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125999326: FileFormat = FileFormat {
    id: 125_999_326,
    puid: "wikidata/125999326",
    name: "Sibelius Score 7",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
