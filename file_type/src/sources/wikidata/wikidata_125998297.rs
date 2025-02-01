use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125998297: FileFormat = FileFormat {
    id: 125_998_297,
    puid: "wikidata/125998297",
    name: "Sibelius Score 4",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
