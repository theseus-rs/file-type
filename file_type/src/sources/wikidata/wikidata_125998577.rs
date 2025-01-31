use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125998577: FileFormat = FileFormat {
    id: 125_998_577,
    puid: "wikidata/125998577",
    name: "Sibelius Score 5",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
