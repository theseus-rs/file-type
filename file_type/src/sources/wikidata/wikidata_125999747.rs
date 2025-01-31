use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125999747: FileFormat = FileFormat {
    id: 125_999_747,
    puid: "wikidata/125999747",
    name: "Sibelius Score 7.5-8.0",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
