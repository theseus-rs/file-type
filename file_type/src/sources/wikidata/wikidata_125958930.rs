use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125958930: FileFormat = FileFormat {
    id: 125_958_930,
    puid: "wikidata/125958930",
    name: "Sibelius Score 1.2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
