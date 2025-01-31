use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125959218: FileFormat = FileFormat {
    id: 125_959_218,
    puid: "wikidata/125959218",
    name: "Sibelius Score 2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
