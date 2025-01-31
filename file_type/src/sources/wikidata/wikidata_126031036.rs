use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126031036: FileFormat = FileFormat {
    id: 126_031_036,
    puid: "wikidata/126031036",
    name: "Sibelius Score 2023.5-2023.8",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
