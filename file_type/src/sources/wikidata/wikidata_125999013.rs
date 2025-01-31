use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125999013: FileFormat = FileFormat {
    id: 125_999_013,
    puid: "wikidata/125999013",
    name: "Sibelius Score 6",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
